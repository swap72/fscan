use clap::{Parser, Subcommand, ValueEnum};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use sysinfo::{System, Process};

#[derive(Parser, Debug)]
#[command(
    name = "fscan",
    version,
    about = "Fast directory & process scanner: report large files/folders or top memory processes."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// about and credits
    About,

    /// Scan a directory
    Scan(ScanArgs),

    /// Scan running processes sorted by memory usage
    P, // 👈 now just `p`
}

#[derive(Parser, Debug)]
struct ScanArgs {
    /// Path to scan
    path: String,

    /// Output format: csv, json, or summary
    #[arg(value_enum)]
    output: OutputFormat,

    /// Minimum size filter: skip-64, skip-128, skip-256, skip-512, skip-1024, skip-2048 (in MB)
    #[arg()]
    skip: Option<SkipLimit>,

    /// Exclude empty folders from final output
    #[arg(long)]
    exclude_empty: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Csv,
    Json,
    Summary,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SkipLimit {
    Skip64,
    Skip128,
    Skip256,
    Skip512,
    Skip1024,
    Skip2048,
}

#[derive(Serialize)]
struct FileEntry {
    path: String,
    size_bytes: u64,
    size_human: String,
    kind: String,
}

#[derive(Serialize)]
struct ProcessEntry {
    pid: i32,
    name: String,
    memory_mb: f64,
}

fn format_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, units[unit])
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::About => {
            println!("📂 fscan");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("A fast, parallel directory scanner that reports only large files/folders.");
            println!("Also scans running processes by memory usage.");
            println!("GitHub: https://github.com/swap72/fscan");
            println!("⭐️ Please star and fork this project to help and support it! ❤️");
            println!("License: MIT");
            println!("Built with Rust 🦀⚙🚀");
            println!("Created by: Swapnil Mishra (https://swapnil.bio.link/)");
        }

        Commands::Scan(args) => {
            run_scan(args);
        }

        Commands::P => {
            scan_processes();
        }
    }
}

fn run_scan(cli: &ScanArgs) {
    let min_size = match cli.skip {
        Some(SkipLimit::Skip64) => Some(64 * 1024 * 1024),
        Some(SkipLimit::Skip128) => Some(128 * 1024 * 1024),
        Some(SkipLimit::Skip256) => Some(256 * 1024 * 1024),
        Some(SkipLimit::Skip512) => Some(512 * 1024 * 1024),
        Some(SkipLimit::Skip1024) => Some(1024 * 1024 * 1024),
        Some(SkipLimit::Skip2048) => Some(2048 * 1024 * 1024),
        None => None,
    };

    if let Some(limit) = min_size {
        println!(
            "Including only files/folders larger than: {}",
            format_size(limit)
        );
    }

    let file_sizes: Arc<Mutex<HashMap<PathBuf, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let dir_sizes: Arc<Mutex<HashMap<PathBuf, u64>>> = Arc::new(Mutex::new(HashMap::new()));

    WalkDir::new(&cli.path)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|e| e.file_type().is_file())
        .for_each(|entry| {
            let path = entry.path().to_path_buf();
            let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

            if let Some(limit) = min_size {
                if size <= limit {
                    return;
                }
            }

            {
                let mut fsizes = file_sizes.lock().unwrap();
                fsizes.insert(path.clone(), size);
            }

            let mut current = path.parent();
            while let Some(parent) = current {
                {
                    let mut dsizes = dir_sizes.lock().unwrap();
                    *dsizes.entry(parent.to_path_buf()).or_insert(0) += size;
                }
                current = parent.parent();
            }
        });

    let fsizes = file_sizes.lock().unwrap();
    let dsizes = dir_sizes.lock().unwrap();

    let mut entries: Vec<FileEntry> = Vec::with_capacity(fsizes.len() + dsizes.len());

    for (path, size) in fsizes.iter() {
        entries.push(FileEntry {
            path: path.display().to_string(),
            size_bytes: *size,
            size_human: format_size(*size),
            kind: "File".to_string(),
        });
    }

    for (path, size) in dsizes.iter() {
        entries.push(FileEntry {
            path: path.display().to_string(),
            size_bytes: *size,
            size_human: format_size(*size),
            kind: "Directory".to_string(),
        });
    }

    if cli.exclude_empty {
        println!("Excluding empty directories from output.");
        entries.retain(|e| !(e.kind == "Directory" && e.size_bytes == 0));
    }

    entries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    for entry in &entries {
        println!(
            "{:>10} [{}] - {}",
            entry.size_human, entry.kind, entry.path
        );
    }

    match cli.output {
        OutputFormat::Csv => {
            if let Ok(mut file) = File::create("output.csv") {
                writeln!(file, "path,size_bytes,size_human,kind").unwrap();
                for e in &entries {
                    writeln!(
                        file,
                        "\"{}\",{},{},{}",
                        e.path, e.size_bytes, e.size_human, e.kind
                    )
                    .unwrap();
                }
                println!("Exported to output.csv");
            }
        }
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(&entries) {
                fs::write("output.json", json).unwrap();
                println!("Exported to output.json");
            }
        }
        OutputFormat::Summary => {
            let total_files = fsizes.len();
            let total_dirs = dsizes.len();
            let total_file_size: u64 = fsizes.values().sum();
            let total_dir_size: u64 = dsizes.values().sum();
            let total_size = total_file_size + total_dir_size;

            println!("\nScan Summary:");
            println!("-------------");
            println!("Total files: {}", total_files);
            println!("Total folders: {}", total_dirs);
            println!("Total size: {}", format_size(total_size));
            println!();

            let mut dir_list: Vec<_> = dsizes.iter().collect();
            dir_list.sort_by(|a, b| b.1.cmp(a.1));
            println!("Top 5 folders:");
            for (i, (path, size)) in dir_list.iter().take(5).enumerate() {
                println!("{}. {} ({})", i + 1, path.display(), format_size(**size));
            }

            println!();

            let mut file_list: Vec<_> = fsizes.iter().collect();
            file_list.sort_by(|a, b| b.1.cmp(a.1));
            println!("Top 5 files:");
            for (i, (path, size)) in file_list.iter().take(5).enumerate() {
                println!("{}. {} ({})", i + 1, path.display(), format_size(**size));
            }
        }
    }
}

fn scan_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    let total_memory: f64 = processes.iter().map(|p| p.memory() as f64 / 1024.0).sum();

    println!("\n📊 Running Processes by Memory Usage");
    println!("┌───────┬────────────────────────────────────────┬─────────────┐");
    println!("│ {:<5} │ {:<42} │ {:>11} │", "PID", "Process Name", "Memory MB");
    println!("├───────┼────────────────────────────────────────┼─────────────┤");

    for process in &processes {
        let pid = process.pid();
        let name = process.name();
        let memory_mb = process.memory() as f64 / 1024.0;

        println!(
            "│ {:<5} │ {:<42} │ {:>11.2} │",
            pid, name, memory_mb
        );
    }

    println!("└───────┴────────────────────────────────────────┴─────────────┘");

    println!(
        "\n📝 Summary → Total Processes: {}, Total Memory Used: {:.2} MB",
        processes.len(),
        total_memory
    );
}
