# 🧮 countwords-rs

A simple and efficient **Word Counter CLI tool** written in Rust.  
It analyzes text files and counts the number of words contained within.

## 🚀 Features
- Count words from any text file
- Export counted words and frequencies to a **CSV file**
- Simple command-line interface using **Clap**

## 📦 Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/andrejkoller/countwords-rs.git
cd countwords-rs
cargo build --release
```

## ⚙️ Usage

```bash
cargo run -- <file_path>
```

```bash
cargo run -- <file_path> --output <output_file>
```
