# Vaultgrim - Installation Guide

## Method 1: npm (Recommended)

```bash
npm install -g vaultgrim
vaultgrim
```

---

## Method 2: One-Line Install

### Linux / Termux / macOS:
```bash
curl -sL https://bit.ly/vaultgrim | bash
```

### Alternative short URLs:
```bash
curl -sL https://tinyurl.com/vaultgrim | bash
```

---

## Method 3: Direct Download

### Termux:
```bash
curl -o /data/data/com.termux/files/usr/bin/vaultgrim https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh
chmod +x /data/data/com.termux/files/usr/bin/vaultgrim
vaultgrim
```

### Linux:
```bash
sudo curl -o /usr/local/bin/vaultgrim https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh
sudo chmod +x /usr/local/bin/vaultgrim
vaultgrim
```

### macOS:
```bash
sudo curl -o /usr/local/bin/vaultgrim https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh
sudo chmod +x /usr/local/bin/vaultgrim
vaultgrim
```

---

## Method 4: Build from Source

### Termux:
```bash
pkg update && pkg install -y rust git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/wahaca9693/vaultgrim.git
cd vaultgrim
cargo build --release
./target/release/vaultgrim menu
```

---

## Method 5: Binary Release (Coming Soon)

Download pre-built binary for your platform:
- Linux x86_64
- Android ARM64
- macOS
- Windows (WSL required)

---

## Quick Start

After installation:

```bash
vaultgrim
```

Follow the interactive menu:
1. Select file type
2. Choose encryption algorithm
3. Enter file path
4. Done!

---

## Troubleshooting

### Permission denied:
```bash
chmod +x vaultgrim
```

### Command not found:
Add to ~/.bashrc or ~/.zshrc:
```bash
export PATH="$PATH:$HOME/.npm-global/bin"
```

---

## Uninstall

```bash
npm uninstall -g vaultgrim
```

Or:
```bash
sudo rm /usr/local/bin/vaultgrim
sudo rm /usr/local/bin/vaultgrim-decrypt
```
