# simpless

A lightweight, no-nonsense screen capture CLI utility written in Rust.

Built with performance in mind, powered by `xcap`. Designed for fast captures with direct clipboard and disk workflows.

---

## Features

- **Blazing Fast:** Written in Rust with minimal runtime overhead.
- **Auto Naming:** Generates timestamped filenames by default.
- **Clipboard Integration:** Instantly pipes screen captures to the system clipboard.
- **Wayland & X11 Ready:** Seamless capture support across Linux display servers.

---

## Installation

### Arch Linux (PKGBUILD)

Build and install locally:

```bash
git clone https://github.com/thegears/simpless.git
cd simpless
makepkg -sic
```

### Any Linux Distribution (Cargo)

Prerequisites: `cargo`, `rustc`, and `libxcb` development headers.

```bash
cargo install --git https://github.com/thegears/simpless.git
```

---

## Usage

```bash
# Capture full screen and copy to clipboard.
simpless

# Capture full screen and save as a file.
simpless --save=~/Desktop

# Show help menu and flags
simpless --help
```

---

## Dependencies

- **Build:** `cargo`, `rustc`
- **Runtime:** `gcc-libs`, `libxcb`

---

## License

This project is licensed under the [MIT License](LICENSE).
