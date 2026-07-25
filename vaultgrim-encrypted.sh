#!/bin/bash
# ============================================================
#  Vaultgrim - Self-Encrypting Binary
#  This script decrypts and runs the encrypted binary
# ============================================================

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Encrypted binary location
ENCRYPTED_BIN="${SCRIPT_DIR}/vaultgrim.bin.enc"
DECRYPTED_BIN="${SCRIPT_DIR}/vaultgrim_tmp"
KEY_FILE="${SCRIPT_DIR}/.key"

# Cleanup on exit
cleanup() {
    if [ -f "${DECRYPTED_BIN}" ]; then
        shred -u "${DECRYPTED_BIN}" 2>/dev/null || rm -f "${DECRYPTED_BIN}"
    fi
}
trap cleanup EXIT

# Check if encrypted binary exists
if [ ! -f "${ENCRYPTED_BIN}" ]; then
    echo "Error: Encrypted binary not found!"
    echo "Please encrypt the binary first: vaultgrim --encrypt-binary"
    exit 1
fi

# Generate or load key
if [ ! -f "${KEY_FILE}" ]; then
    echo "First run - generating encryption key..."
    openssl rand -base64 32 > "${KEY_FILE}"
    chmod 600 "${KEY_FILE}"
fi

# Decrypt the binary
openssl enc -aes-256-cbc -d -salt -pbkdf2 -in "${ENCRYPTED_BIN}" -out "${DECRYPTED_BIN}" -pass file:"${KEY_FILE}" 2>/dev/null

# Make executable
chmod +x "${DECRYPTED_BIN}"

# Run the decrypted binary with original arguments
exec "${DECRYPTED_BIN}" "$@"
