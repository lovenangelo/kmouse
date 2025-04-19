![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![Rust Version](https://img.shields.io/badge/rust-stable-orange)

# Kmouse

Kmouse is a keyboard-controlled mouse application that creates a transparent overlay allowing you to navigate and click using only keyboard inputs. Perfect for users who prefer keyboard navigation or have mobility impairments affecting mouse usage.

![Kmouse Demo](docs/images/kmouse-demo.gif)

---

## 🚀 Releases

The latest release is [`v0.1.0-beta`](https://github.com/lovenangelo/kmouse/releases) — a fast, keyboard-powered way to click anywhere on screen faster. Includes support for Linux (X11) and an intuitive grid overlay for precise navigation.

> Note: This is a **beta release**. Expect some rough edges — feedback and contributions are welcome!

---

## Features

- **Keyboard-Driven Navigation**: Control your mouse cursor entirely from the keyboard
- **Two-Level Selection**: First select a grid cell, then a precise location within that cell
- **Transparent Overlay**: Non-intrusive interface that appears only when activated
- **Customizable**: Configurable key bindings, grid size, and appearance
- **X11 Support**: Works with X11-based Linux desktop environments
- **Cross-platform support is in progress!**

---

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or newer)
- X11 development libraries
- Cargo

### From Source

```bash
# Clone the repository
git clone https://github.com/lovenangelo/kmouse.git
cd kmouse

# Build in release mode
cargo build --release

# Install (optional)
cargo install --path .

---

### 🤝 Contributing

Kmouse is proudly open source and looking for contributors! Whether you’re fixing bugs, improving UX, adding features, or working on cross-platform support — **your help is welcome**.

Check out the [issues page](https://github.com/lovenangelo/kmouse/issues) for ideas on where to start. You can also tag issues with `good first issue` or `help wanted` to guide new contributors.

> Got an idea or feedback? Open an issue or submit a pull request — we’d love to hear from you.
