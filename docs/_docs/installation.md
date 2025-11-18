---
layout: doc
title: Installation
---

## Requirements

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Discord Account** - You'll need a Discord user token
- **Terminal Emulator** - Any modern terminal works, Kitty recommended for image support

### Platform-Specific Requirements

#### macOS
- Xcode Command Line Tools: `xcode-select --install`
- Keychain access for secure token storage

#### Linux
- Build essentials: `sudo apt install build-essential pkg-config`
- GNOME Keyring: `sudo apt install gnome-keyring libsecret-1-dev`
- D-Bus session running

#### Windows
- Visual Studio Build Tools
- Windows Credential Manager (built-in)

## Installation Methods

### From Source (Recommended)

Clone and build from the repository:

```bash
# Clone the repository
git clone https://github.com/yourusername/remycord.git
cd remycord

# Build and install
cargo install --path .
```

The binary will be installed to `~/.cargo/bin/remycord`.

### Using Cargo

Install directly from crates.io (once published):

```bash
cargo install remycord
```

### Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/yourusername/remycord/releases).

#### Linux
```bash
# Download and extract
wget https://github.com/yourusername/remycord/releases/latest/download/remycord-linux.tar.gz
tar xzf remycord-linux.tar.gz

# Move to PATH
sudo mv remycord /usr/local/bin/
```

#### macOS
```bash
# Download and extract
curl -LO https://github.com/yourusername/remycord/releases/latest/download/remycord-macos.tar.gz
tar xzf remycord-macos.tar.gz

# Move to PATH
sudo mv remycord /usr/local/bin/
```

#### Windows
Download `remycord-windows.zip` from releases and extract to a directory in your PATH.

## Verification

Verify the installation:

```bash
remycord --version
```

## Next Steps

After installation, you need to:

1. [Set up your Discord token]({{ '/docs/token-setup/' | relative_url }})
2. [Configure remycord]({{ '/docs/configuration/' | relative_url }})
3. [Learn the keybindings]({{ '/docs/keybindings/' | relative_url }})

## Troubleshooting

### Cargo not found
Make sure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.).

### Build failures on Linux
Install required dependencies:

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel

# Arch
sudo pacman -S base-devel openssl
```

### Permission denied (macOS)
If you get permission denied when moving to `/usr/local/bin`, use:

```bash
sudo chown -R $(whoami) /usr/local/bin
```
