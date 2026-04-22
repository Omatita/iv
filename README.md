# iv

A minimalist image viewer written in Rust.

## Requirements

- Rust 
- System dependencies (Linux):
  - **Arch Linux:** `sudo pacman -S pkgconf`
  - **Debian/Ubuntu:** `sudo apt install pkg-config libx11-dev libasound2-dev`

## Installation

1. Build
   ```bash
   git clone https://github.com/Omatita/iv.git
   cd iv
   cargo build --release
   ```

2. Install the binary
   ```bash
   sudo cp target/release/iv /usr/local/bin/
   ```

## Usage

Pass the path of the image you want to open as an argument:

```bash
iv /path/to/image.jpg
```
