#!/bin/bash
# Vaultgrim - Universal Install Script
# Installs from source code - no releases needed

set -e

REPO="https://github.com/wahaca9693/vaultgrim.git"
INSTALL_DIR="${HOME}/vaultgrim"

echo ""
echo "=========================================="
echo "  Vaultgrim - Universal Installer"
echo "  Installs from source"
echo "=========================================="
echo ""

# 1. Install Rust
echo "[1/5] Installing Rust..."
if command -v cargo &> /dev/null; then
    echo "OK Rust already installed: $(cargo --version)"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    [ -f "${HOME}/.cargo/env" ] && source "${HOME}/.cargo/env"
    echo "OK Rust installed"
fi

# 2. Download Source
echo "[2/5] Downloading source code..."
if [ -d "${INSTALL_DIR}" ]; then
    cd "${INSTALL_DIR}" && git pull origin main
else
    git clone "${REPO}" "${INSTALL_DIR}" && cd "${INSTALL_DIR}"
fi
echo "OK Source downloaded"

# 3. Setup Mirror
echo "[3/5] Configuring Cargo..."
if [ ! -f "${HOME}/.cargo/config.toml" ]; then
    mkdir -p "${HOME}/.cargo"
    cat > "${HOME}/.cargo/config.toml" << 'MIRROR'
[source.crates-io]
replace-with = "ustc"
[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
MIRROR
fi
echo "OK Mirror configured"

# 4. Build
echo "[4/5] Building..."
source "${HOME}/.cargo/env" 2>/dev/null || true
cargo build --release 2>&1 | tail -3
echo "OK Build complete"

# 5. Test
echo "[5/5] Testing..."
VAULTGRIM_PATH="${INSTALL_DIR}/target/release/vaultgrim"
chmod +x "${VAULTGRIM_PATH}"
echo ""
echo "=========================================="
echo "INSTALLATION COMPLETE"
echo "=========================================="
echo ""
echo "Location: ${VAULTGRIM_PATH}"
echo ""
echo "Commands:"
"${VAULTGRIM_PATH}" --help

# Alias
RC="${HOME}/.bashrc"
[ -n "$ZSH_VERSION" ] && RC="${HOME}/.zshrc"
grep -q "vaultgrim=" "${RC}" || echo "alias vaultgrim='${VAULTGRIM_PATH}'" >> "${RC}"
echo ""
echo "Run: source ${RC} && vaultgrim --help"
echo ""
