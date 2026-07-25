#!/bin/bash
# ============================================================
#  Encrypt Vaultgrim Binary
# ============================================================

BIN_FILE="${1:-./target/release/vaultgrim}"
OUTPUT_FILE="vaultgrim.bin.enc"
KEY_FILE=".key"

echo "============================================"
echo "  Vaultgrim Binary Encryption Tool"
echo "============================================"
echo ""

if [ ! -f "${BIN_FILE}" ]; then
    echo "Error: Binary not found at ${BIN_FILE}"
    exit 1
fi

if ! command -v openssl &> /dev/null; then
    echo "Error: openssl not found!"
    exit 1
fi

echo "Binary: ${BIN_FILE}"
echo "Size: $(ls -lh "${BIN_FILE}" | awk '{print $5}')"
echo ""

echo "Generating encryption key..."
openssl rand -base64 32 > "${KEY_FILE}"
chmod 600 "${KEY_FILE}"

echo "Encrypting binary..."
openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 -in "${BIN_FILE}" -out "${OUTPUT_FILE}" -pass file:"${KEY_FILE}"

echo ""
echo "============================================"
echo "  ENCRYPTION COMPLETE!"
echo "============================================"
echo ""
echo "Encrypted: ${OUTPUT_FILE}"
echo "Key: ${KEY_FILE}"
echo ""
echo "Run: ./vaultgrim-encrypted.sh --help"
