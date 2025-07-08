# Scan a directory and print a summary, skipping files smaller than 1024 Megabytes
fscan scan c:\users summary skip1024

# Exclude empty folders from the scan
fscan scan /home/user/ summary --exclude-empty

# Display about information and credits
fscan about

# Show help with available flags and options
fscan --help


# Info :
You can compile to native binary or can directly [download the tool from here,](https://github.com/swap72/fscan/raw/refs/heads/main/target/release/fscan.exe) register it on the path of you system and start using it (Windows 10 and up)

# ffscan

[![Crates.io](https://img.shields.io/crates/v/ffscan.svg)](https://crates.io/crates/ffscan)
[![Downloads](https://img.shields.io/crates/d/ffscan.svg)](https://crates.io/crates/ffscan)


## ⚙️ Output Formats

- **csv**: Save results to `output.csv`
- **json**: Save results to `output.json`
- **summary**: Print a human-readable summary to the console

---

## 📂 Example output

**2.50 GB [Directory] - C:\Users\Tushar\Documents\Projects**  
**1.20 GB [File] - C:\Users\Tushar\Videos\movie.mp4**


---

## 📁 Project Structure
```
fscan/
├── Cargo.toml           # Project metadata & dependencies
├── Cargo.lock           # Locked dependency versions (auto-generated)
├── LICENSE              # LICENSE.txt
├── README.md            # 📄 Project documentation (GitHub flavored)
├── .gitignore           # Ignore build artifacts & output files
├── output.csv           # Example output file (should be gitignored)
├── output.json          # Example output file (should be gitignored)
├── src/
│   ├── main.rs          # Main entry point: parses CLI & calls logic
│   ├── cli.rs           # (Optional) CLI parsing module if you split
│   ├── scanner.rs       # (Optional) Scanning logic module
│   └── utils.rs         # (Optional) Utility functions (e.g., format_size)
└── .github/
    └── workflows/
        └── rust.yml     # (Optional) CI workflow for testing/building
```
---

## 📝 License

This project is licensed under the MIT License.  
See [LICENSE](https://github.com/swap72/fscan/blob/main/LICENSE.txt.txt) for details.

---

## 🙌 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

## 💖 Show your support
🌱 Feel free to modify and distribute this CLI tool  
⭐️ Star or fork this repo on GitHub if you find it useful!  
🔗 [Formal Portfolio](https://swap72.github.io/portfolio/)  
🔗 [Not so formal Portfolio](http://swapnil.bio.link/)  
🚀 Built with ❤️ and Rust 🦀⚙️
