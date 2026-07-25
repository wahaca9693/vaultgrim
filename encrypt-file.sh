#!/bin/bash
# ============================================================
#  Vaultgrim - Simple File Encryption
#  Just select number + file = done!
# ============================================================

set +e

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
    1) ALGO="AES-256-GCM" ;;
    2) ALGO="ChaCha20-Poly1305" ;;
    3) ALGO="Camellia-256-GCM" ;;
    4) ALGO="SM4-GCM" ;;
    5) ALGO="Twofish-256-GCM" ;;
    6) ALGO="ARIA-256-GCM" ;;
    7) ALGO="Serpent-256-GCM" ;;
    *) ALGO="AES-256-GCM" ;;
esac

echo ""
echo -n "Enter file path: "
read FILE

FILE=$(echo "$FILE" | sed 's/\r//g' | xargs)

if [ -z "$FILE" ]; then
    echo "Error: No file path entered!"
    exit 1
fi

echo ""
echo "Checking file: $FILE"

if [ ! -e "$FILE" ]; then
    echo "File not found. Searching..."
    
    if [ -e "/storage/emulated/0/$FILE" ]; then
        FILE="/storage/emulated/0/$FILE"
    elif [ -e "$HOME/$FILE" ]; then
        FILE="$HOME/$FILE"
    elif [ -e "./$FILE" ]; then
        FILE="./$FILE"
    else
        echo "Error: File not found: $FILE"
        exit 1
    fi
fi

if [ ! -f "$FILE" ]; then
    echo "Error: Not a file: $FILE"
    exit 1
fi

REAL_PATH=$(realpath "$FILE" 2>/dev/null || echo "$FILE")

echo "Encrypting: $REAL_PATH"
echo "Algorithm: $ALGO"
echo ""

KEY_FILE="${REAL_PATH}.key"
openssl rand -base64 32 > "$KEY_FILE" 2>/dev/null

OUTPUT="${REAL_PATH}.enc"
openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 -in "$REAL_PATH" -out "$OUTPUT" -pass file:"$KEY_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "════════════════════════════════════════════════════════════"
    echo "  ENCRYPTION COMPLETE!"
    echo "════════════════════════════════════════════════════════════"
    echo ""
    echo "  Original:   $REAL_PATH"
    echo "  Encrypted:  $OUTPUT"
    echo "  Key:        $KEY_FILE"
    echo "  Algorithm:  $ALGO"
    echo ""
    echo "  IMPORTANT: Keep the key file safe!"
else
    echo "Error: Encryption failed!"
    rm -f "$KEY_FILE"
    exit 1
fi
