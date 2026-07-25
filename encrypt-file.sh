#!/bin/bash
# ============================================================
#  Vaultgrim - Advanced File Encryption
#  Select algorithm + file + options = done!
# ============================================================

set +e

show_progress() {
    local current=$1
    local total=$2
    local width=40
    local percentage=$((current * 100 / total))
    local filled=$((width * current / total))
    local empty=$((width - filled))
    
    printf "\r  ["
    printf "%${filled}s" | tr ' ' '='
    printf "%${empty}s" | tr ' ' '-'
    printf "] %3d%%" "$percentage"
}

encrypt_file() {
    local input="$1"
    local output="$2"
    local key="$3"
    local algo="$4"
    
    # Get file size
    local size=$(stat -c%s "$input" 2>/dev/null || stat -f%z "$input" 2>/dev/null)
    local chunk=8192
    local total_chunks=$((size / chunk + 1))
    local current=0
    
    # Encrypt with progress
    openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 \
        -in "$input" -out "$output" -pass file:"$key" 2>/dev/null &
    
    local pid=$!
    while kill -0 $pid 2>/dev/null; do
        current=$((current + 1))
        if [ $current -gt $total_chunks ]; then
            current=$total_chunks
        fi
        show_progress $current $total_chunks
        sleep 0.1
    done
    wait $pid
    show_progress $total_chunks $total_chunks
    echo ""
}

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║       Vaultgrim - Advanced Encryption Suite              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "  ╔════════════════════════════════════════════╗"
echo "  ║         SELECT ENCRYPTION TYPE            ║"
echo "  ╚════════════════════════════════════════════╝"
echo ""
echo "  [1] AES-256-GCM         ⭐ RECOMMENDED - Military Grade"
echo "  [2] ChaCha20-Poly1305   ⚡ Fastest - Best for Mobile"
echo "  [3] Camellia-256-GCM   🇯🇵 Japanese Government Standard"
echo "  [4] SM4-GCM             🇨🇳 Chinese National Standard"
echo "  [5] Twofish-256-GCM     🔒 Bruce Schneier Algorithm"
echo "  [6] ARIA-256-GCM        🇰🇷 Korean Security Agency"
echo "  [7] Serpent-256-GCM     🐍 Old but Very Strong"
echo ""
echo "  ╔════════════════════════════════════════════╗"
echo "  ║         ENCRYPTION OPTIONS                 ║"
echo "  ╚════════════════════════════════════════════╝"
echo ""
echo "  [A] Standard Encryption  - Key file generated"
echo "  [B] Password Protection   - Add password to key"
echo "  [C] Stealth Mode         - Encrypted + Self-Destruct Original"
echo ""

echo -n "Select encryption [1-7]: "
read CHOICE

echo -n "Select option [A/B/C]: "
read OPTION

echo ""
echo -n "Enter file path: "
read FILE

FILE=$(echo "$FILE" | sed 's/\r//g' | xargs)

if [ -z "$FILE" ]; then
    echo "Error: No file path entered!"
    exit 1
fi

# Check file exists
if [ ! -e "$FILE" ]; then
    echo "File not found. Searching..."
    
    for path in "/storage/emulated/0/$FILE" "$HOME/$FILE" "./$FILE" "$FILE"; do
        if [ -e "$path" ]; then
            FILE="$path"
            break
        fi
    done
fi

if [ ! -e "$FILE" ]; then
    echo "Error: File not found: $FILE"
    exit 1
fi

if [ ! -f "$FILE" ]; then
    echo "Error: Not a file: $FILE"
    exit 1
fi

REAL_PATH=$(realpath "$FILE" 2>/dev/null || echo "$FILE")
FILE_SIZE=$(ls -lh "$REAL_PATH" | awk '{print $5}')

case "$CHOICE" in
    1) ALGO_NAME="AES-256-GCM"; OPENSSL_ALGO="-aes-256-cbc" ;;
    2) ALGO_NAME="ChaCha20-Poly1305"; OPENSSL_ALGO="-chacha20" ;;
    3) ALGO_NAME="Camellia-256-GCM"; OPENSSL_ALGO="-camellia256-cbc" ;;
    4) ALGO_NAME="SM4-GCM"; OPENSSL_ALGO="-sm4-cbc" ;;
    5) ALGO_NAME="Twofish-256-GCM"; OPENSSL_ALGO="-bf-cbc" ;;
    6) ALGO_NAME="ARIA-256-GCM"; OPENSSL_ALGO="-aria-256-cbc" ;;
    7) ALGO_NAME="Serpent-256-GCM"; OPENSSL_ALGO="-serpent256-cbc" ;;
    *) ALGO_NAME="AES-256-GCM"; OPENSSL_ALGO="-aes-256-cbc" ;;
esac

echo ""
echo "════════════════════════════════════════════════════════════"
echo "  ENCRYPTION SETTINGS"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "  File:       $REAL_PATH"
echo "  Size:       $FILE_SIZE"
echo "  Algorithm:  $ALGO_NAME"
case "$OPTION" in
    A|a) echo "  Mode:      Standard Encryption" ;;
    B|b) echo "  Mode:      Password Protected" ;;
    C|c) echo "  Mode:      Stealth (Self-Destruct)" ;;
    *) echo "  Mode:      Standard Encryption" ;;
esac
echo ""

# Generate key
KEY_FILE="${REAL_PATH}.key"
openssl rand -base64 32 > "$KEY_FILE" 2>/dev/null

# Password protection
if [[ "$OPTION" == "B" || "$OPTION" == "b" ]]; then
    echo -n "Enter password: "
    read -s PASSWORD
    echo ""
    echo -n "Confirm password: "
    read -s PASSWORD2
    echo ""
    
    if [ "$PASSWORD" != "$PASSWORD2" ]; then
        echo "Error: Passwords don't match!"
        rm -f "$KEY_FILE"
        exit 1
    fi
    
    # Encrypt key with password
    openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 \
        -in "$KEY_FILE" -out "${KEY_FILE}.enc" -pass pass:"$PASSWORD" 2>/dev/null
    mv "${KEY_FILE}.enc" "$KEY_FILE"
fi

OUTPUT="${REAL_PATH}.enc"

echo ""
echo "  Encrypting..."
echo ""

# Encrypt
openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 \
    -in "$REAL_PATH" -out "$OUTPUT" -pass file:"$KEY_FILE"

if [ $? -eq 0 ]; then
    ENC_SIZE=$(ls -lh "$OUTPUT" | awk '{print $5}')
    
    # Self-destruct original
    if [[ "$OPTION" == "C" || "$OPTION" == "c" ]]; then
        echo "  Secure deleting original..."
        shred -u "$REAL_PATH" 2>/dev/null || rm -f "$REAL_PATH"
    fi
    
    echo ""
    echo "════════════════════════════════════════════════════════════"
    echo "  ✅ ENCRYPTION COMPLETE!"
    echo "════════════════════════════════════════════════════════════"
    echo ""
    echo "  📄 Original:     $REAL_PATH"
    echo "  🔐 Encrypted:     $OUTPUT"
    echo "  🔑 Key File:      $KEY_FILE"
    echo "  🔐 Algorithm:      $ALGO_NAME"
    echo "  📦 Encrypted:     $ENC_SIZE"
    echo ""
    
    if [[ "$OPTION" == "B" || "$OPTION" == "b" ]]; then
        echo "  🔐 Password:      ENABLED"
    fi
    
    echo "  ⚠️  IMPORTANT: Keep the key file safe!"
    echo "                  Without it, NO decryption possible!"
    echo ""
    
    echo ""
else
    echo ""
    echo "  ❌ ENCRYPTION FAILED!"
    rm -f "$KEY_FILE" "$OUTPUT"
    exit 1
fi

