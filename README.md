# Basic scan
fscan scan <PATH> <OUTPUT_FORMAT>

# Example: scan C:\ and show summary, skipping files smaller than 1 GB
fscan scan C:\ summary skip1024

# Exclude empty folders
fscan scan /home/user/ summary --exclude-empty

# Show about and credits
fscan about

# Usage
fscan --help


## ⚙️ Output formats

**csv**: Save results to output.csv
**json**: Save results to output.json
**summary**: Print a readable summary to the console

---

## 📂 Example output

**2.50 GB [Directory] - C:\Users\Tushar\Documents\Projects**  
**1.20 GB [File] - C:\Users\Tushar\Videos\movie.mp4**


---

## 📁 Project Structure
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

---

## 📝 License

This project is licensed under the MIT License.  
See [LICENSE](LICENSE) for details.

---

## 🙌 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

## 💖 Show your support
🌱 Feel free to modify and distribute this CLI tool  
⭐️ Star or fork this repo on GitHub if you find it useful!  
🔗 [Formal Portfolio](https://swapnil.bio.link/)  
🔗 [Not so formal Portfolio](https://swap72.github.io/portfolio/)  
🚀 Built with ❤️ and Rust 🦀⚙️
