**Pandora** is an emerging digital forensics toolkit written in Rust. Currently in early development, it aims to become a versatile "Swiss Army knife" for forensic analysts and CTF participants by offering a growing collection of tools to uncover hidden or obscured data using well-known forensic techniques.

## 📦 Features

- Detects file types using header bytes (magic numbers)
- Matches multiple possible file types for the same signature
- Organized modular design with a menu-based CLI
- Easy-to-extend TOML database of file signatures

#### ⚠️ This project is in its early stages of development and has minimal features as of now.

## 🛠️ Usage

1. Clone the repository:

```bash
git clone https://github.com/SauravSreejith/Pandora.git
cd Pandora
```

2. Run the project:

```bash
cargo run
```

## 🧰 Requirements

- Rust & Cargo (latest stable)
- Works on Linux, macOS, and Windows

## 📜 License

MIT License. Do whatever you want, but don't blame me.

## 🤝 Contributing

Pull requests and signature contributions are welcome! If you are trying to extend the file signatures make sure your TOML format follows the existing structure.

## TO-DO List
- Add more collections of tools
- Improve user experience.
- Add a bruteforce script