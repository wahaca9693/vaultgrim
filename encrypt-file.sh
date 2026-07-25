#!/bin/bash
# ============================================================
#  Vaultgrim - Simple File Encryption
#  Just select number + file = done!
# ============================================================

set -e

FILE="$1"

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║          Vaultgrim - Select Encryption Type              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "  Select encryption algorithm by number:"
echo ""
echo "  [1] AES-256-GCM        (Recommended - Default)"
echo "  [2] ChaCha20-Poly1305  (Fast - Mobile/IoT)"
echo "  [3] Camellia-256-GCM   (Japanese Standard)"
echo "  [4] SM4-GCM            (Chinese Standard)"
echo "  [5] Twofish-256-GCM    (Strong Alternative)"
echo "  [6] ARIA-256-GCM       (Korean Standard)"
echo "  [7] Serpent-256-GCM    (Strong Alternative)"
echo ""
echo -n "Enter number [1-7]: "
read CHOICE

case "$CHOICE" in
    1) ALGO="aes-256-gcm" ;;
    2) ALGO="chacha20-poly1305" ;;
    3) ALGO="camellia-256-gcm" ;;
    4) ALGO="sm4-gcm" ;;
    5) ALGO="twofish-256-gcm" ;;
    6) ALGO="aria-256-gcm" ;;
    7) ALGO="serpent-256-gcm" ;;
    *) ALGO="aes-256-gcm" ;;
esac

if [ -z "$FILE" ]; then
    echo -n "Enter file path: "
    read FILE
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File not found: $FILE"
    exit 1
fi

# Generate key automatically
KEY_FILE="${FILE}.key"
openssl rand -base64 32 > "$KEY_FILE"

# Encrypt
OUTPUT="${FILE}.enc"
openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 -in "$FILE" -out "$OUTPUT" -pass file:"$KEY_FILE"

echo ""
echo "════════════════════════════════════════════════════════════"
echo "  ENCRYPTION COMPLETE!"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "  Original:   $FILE"
echo "  Encrypted:  $OUTPUT"
echo "  Key:        $KEY_FILE"
echo "  Algorithm:  $ALGO"
echo ""
echo "  IMPORTANT: Keep the key file safe!"
echo ""
