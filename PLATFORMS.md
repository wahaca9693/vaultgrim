# Vaultgrim - Installation Links for All Platforms

## Quick Install (One Command)

### Main Encryption Tool
```bash
bash <(curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh)
```

### Decrypt Tool
```bash
bash <(curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/decrypt-file.sh)
```

---

## Linux

### Ubuntu / Debian / Linux Mint
```bash
sudo apt update && sudo apt install -y openssl git
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

### Fedora / CentOS / RHEL
```bash
sudo dnf install -y openssl git
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

### Arch Linux / Manjaro
```bash
sudo pacman -S openssl git
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

---

## Android (Termux)

```bash
pkg update && pkg install -y openssl git
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

---

## macOS

```bash
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

---

## Windows

### WSL (Windows Subsystem for Linux)
```bash
wsl --install
# Then in WSL:
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

### Git Bash
```bash
# Install Git from git-scm.com, then:
curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh -o encrypt-file.sh
chmod +x encrypt-file.sh
./encrypt-file.sh
```

---

## Direct Links

| Tool | Link |
|------|------|
| Encrypt | https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh |
| Decrypt | https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/decrypt-file.sh |
| Python Wrapper | https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/vaultgrim.py |
| Install Script | https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/VAULTGRIM-INSTALL.sh |

---

## All Files

| File | Description |
|------|-------------|
| encrypt-file.sh | Smart encryption tool |
| decrypt-file.sh | Decryption tool |
| vaultgrim.py | Python wrapper |
| vaultgrim-encrypted.sh | Encrypted binary runner |
| encrypt-binary.sh | Binary encryption |
| VAULTGRIM-INSTALL.sh | Full installation |
| PLATFORMS.md | This file |

---

## Build from Source (Rust)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
git clone https://github.com/wahaca9693/vaultgrim.git
cd vaultgrim
cargo build --release
./target/release/vaultgrim menu
```

---

## Quick Commands

| Action | Command |
|--------|---------|
| Encrypt | bash <(curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/encrypt-file.sh) |
| Decrypt | bash <(curl -sL https://raw.githubusercontent.com/wahaca9693/vaultgrim/main/decrypt-file.sh) |
| Interactive | ./vaultgrim menu |
| Benchmark | ./vaultgrim benchmark |

---

## GitHub

https://github.com/wahaca9693/vaultgrim
