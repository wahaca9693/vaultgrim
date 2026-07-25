#!/bin/bash
# Vaultgrim Installer for Termux (Android ARM64)
# Run this script in Termux

echo "🔐 Vaultgrim Installer for Termux"
echo "================================="
echo ""

# Check if we have required tools
command -v curl >/dev/null 2>&1 || { echo "❌ curl not found. Run: pkg install curl"; exit 1; }
command -v git >/dev/null 2>&1 || { echo "❌ git not found. Run: pkg install git"; exit 1; }

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Source cargo env
[ -f ~/.cargo/env ] && source ~/.cargo/env

echo "✅ Rust ready: $(cargo --version)"
echo ""

# Setup Chinese mirror if needed
if [ ! -f ~/.cargo/config.toml ]; then
    echo "🔧 Setting up crates.io mirror..."
    mkdir -p ~/.cargo
    cat > ~/.cargo/config.toml << 'MIRROR'
[source.crates-io]
replace-with = "ustc"

[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
MIRROR
    echo "✅ Mirror configured"
fi

echo ""

# Create vaultgrim directory
cd ~
mkdir -p vaultgrim
cd vaultgrim

# Clone or update repo
if [ -d ".git" ]; then
    echo "📥 Updating Vaultgrim..."
    git pull
else
    echo "📥 Downloading Vaultgrim source..."
    git clone https://github.com/wahaca9693/vaultgrim.git .
fi

echo ""
echo "🔨 Building Vaultgrim (first time may take 5-10 minutes)..."
echo ""

# Build
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "=========================================="
    echo "   Vaultgrim is ready!"
    echo "=========================================="
    echo ""
    echo "Usage:"
    echo "  ~/vaultgrim/target/release/vaultgrim encrypt file.txt"
    echo "  ~/vaultgrim/target/release/vaultgrim decrypt file.txt.enc"
    echo ""
    echo "To use from anywhere, add to PATH:"
    echo '  echo "export PATH=\$PATH:~/vaultgrim/target/release" >> ~/.bashrc'
    echo "  source ~/.bashrc"
    echo ""
    echo "Or create alias:"
    echo '  echo "alias vaultgrim=\"~/vaultgrim/target/release/vaultgrim\"" >> ~/.bashrc'
else
    echo "❌ Build failed!"
    echo "Please report the error above."
fi
