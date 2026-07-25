#!/bin/bash
# ============================================================
#  Vaultgrim - Smart Universal Install Script
#  Waits for ALL libraries to complete before proceeding
# ============================================================

set -e

REPO="https://github.com/wahaca9693/vaultgrim.git"
INSTALL_DIR="${HOME}/vaultgrim"

echo ""
echo "=========================================="
echo "  Vaultgrim - Smart Installer"
echo "  Full dependency download + build"
echo "=========================================="
echo ""

# 1. Install Rust
echo "[1/6] Checking Rust..."
if command -v cargo &> /dev/null; then
    echo "OK Rust ready: $(cargo --version)"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "${HOME}/.cargo/env"
    echo "OK Rust installed"
fi

# 2. Download Source
echo "[2/6] Downloading source..."
cd "${HOME}"
if [ -d "${INSTALL_DIR}" ]; then
    cd "${INSTALL_DIR}" && git pull origin main
else
    git clone "${REPO}" "${INSTALL_DIR}" && cd "${INSTALL_DIR}"
fi
echo "OK Source ready"

# 3. Setup Mirror
echo "[3/6] Configuring Cargo mirror..."
if [ ! -f "${HOME}/.cargo/config.toml" ]; then
    mkdir -p "${HOME}/.cargo"
    cat > "${HOME}/.cargo/config.toml" << 'MIRROR'
[source.crates-io]
replace-with = "ustc"
[source.ustc]
registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"
MIRROR
    echo "OK Mirror configured"
else
    echo "OK Mirror already set"
fi

# 4. Download ALL Dependencies First
echo "[4/6] Downloading ALL dependencies (waiting for completion)..."
echo "This may take 5-15 minutes on first run..."
echo ""

source "${HOME}/.cargo/env" 2>/dev/null || true

# Download dependencies separately and wait for completion
cargo fetch 2>&1

# Wait for cargo to finish completely
wait
echo ""
echo "OK All dependencies downloaded"

# 5. Build
echo "[5/6] Building Vaultgrim..."
echo "Compiling source code..."
echo ""

cargo build --release 2>&1 | while read line; do
    echo "  $line"
done

if [ $? -eq 0 ]; then
    echo ""
    echo "OK Build successful"
else
    echo ""
    echo "ERROR: Build failed"
    exit 1
fi

# 6. Final verification
echo "[6/6] Verifying installation..."
VAULTGRIM_PATH="${INSTALL_DIR}/target/release/vaultgrim"

if [ -f "${VAULTGRIM_PATH}" ]; then
    chmod +x "${VAULTGRIM_PATH}"
    SIZE=$(ls -lh "${VAULTGRIM_PATH}" | awk '{print $5}')
    echo ""
    echo "=========================================="
    echo "  INSTALLATION COMPLETE!"
    echo "=========================================="
    echo ""
    echo "Binary: ${VAULTGRIM_PATH}"
    echo "Size:   ${SIZE}"
    echo ""
    echo "Available commands:"
    echo ""
    "${VAULTGRIM_PATH}" --help
    
    # Create alias
    RC="${HOME}/.bashrc"
    [ -n "$ZSH_VERSION" ] && RC="${HOME}/.zshrc"
    if ! grep -q "vaultgrim=" "${RC}" 2>/dev/null; then
        echo "" >> "${RC}"
        echo "# Vaultgrim" >> "${RC}"
        echo "alias vaultgrim='${VAULTGRIM_PATH}'" >> "${RC}"
        echo "OK Alias added to ${RC}"
    fi
    
    echo ""
    echo "=========================================="
    echo "To use, run:"
    echo "  source ${RC}"
    echo "  vaultgrim --help"
    echo "=========================================="
else
    echo "ERROR: Binary not found"
    exit 1
fi
