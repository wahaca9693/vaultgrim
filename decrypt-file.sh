#!/bin/bash
# ============================================================
#  Vaultgrim - Simple File Decryption
#  Just select file + key = done!
# ============================================================

set -e

FILE="$1"
KEY="$2"

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║          Vaultgrim - Simple Decryption                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

if [ -z "$FILE" ]; then
    echo -n "Enter encrypted file path: "
    read FILE
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File not found: $FILE"
    exit 1
fi

if [ -z "$KEY" ]; then
    echo -n "Enter key file path: "
    read KEY
fi

if [ ! -f "$KEY" ]; then
    echo "Error: Key file not found: $KEY"
    exit 1
fi

# Decrypt
OUTPUT="${FILE%.enc}"
openssl enc -aes-256-cbc -d -pbkdf2 -in "$FILE" -out "$OUTPUT" -pass file:"$KEY"

echo ""
echo "════════════════════════════════════════════════════════════"
echo "  DECRYPTION COMPLETE!"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "  Encrypted:  $FILE"
echo "  Decrypted:  $OUTPUT"
echo ""
