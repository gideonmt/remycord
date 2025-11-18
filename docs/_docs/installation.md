---
layout: default
title: Installation
---

# Installation

## Requirements

- Rust 1.70 or later
- Discord user token
- Modern terminal emulator

## From Source

```bash
git clone https://github.com/yourusername/remycord
cd remycord
cargo build --release
cargo install --path .
```

## Setting Up Your Token

Your Discord token is stored securely using OS-native credential storage.

### macOS

```bash
security add-generic-password -s remycord -a token -w "YOUR_DISCORD_TOKEN"
```

### Linux

```bash
secret-tool store --label="Discord Token" service remycord username token
```

Then enter your token when prompted.

### Windows

```powershell
cmdkey /add:remycord /user:token /pass:YOUR_DISCORD_TOKEN
```

## Getting Your Discord Token

1. Open Discord in your web browser
2. Open Developer Tools (F12)
3. Go to Application → Local Storage → discord.com
4. Find the `token` key and copy its value

**Warning:** Never share your token with anyone.

## Running remycord

```bash
remycord
```

If everything is set up correctly, you'll see a connection message and the main interface.

## Troubleshooting

### Token not found

Make sure you stored the token with the exact service name `remycord` and account name `token`.

### Build errors on Linux

Install required dependencies:

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel
```

### Permission denied

Make sure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
