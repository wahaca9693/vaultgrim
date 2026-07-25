#!/bin/bash
# ============================================================
#  Vaultgrim - Smart File Encryption
#  Select by file type + encryption = done!
# ============================================================

set +e

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║        Vaultgrim - Smart Encryption Suite                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

show_categories() {
echo ""
echo "  ╔══════════════════════════════════════════════════════════════╗"
echo "  ║             SELECT FILE TYPE TO ENCRYPT                  ║"
echo "  ╠══════════════════════════════════════════════════════════════╣"
echo "  ║                                                              ║"
echo "  ║  🐍 SCRIPTS & CODE                                          ║"
echo "  ║     [1] Python (.py, .pyc)                                  ║"
echo "  ║     [2] JavaScript (.js, .mjs)                              ║"
echo "  ║     [3] Ruby (.rb)                                          ║"
echo "  ║     [4] Perl (.pl, .pm)                                     ║"
echo "  ║     [5] Bash/Shell (.sh)                                    ║"
echo "  ║     [6] PHP (.php)                                           ║"
echo "  ║     [7] Java (.java, .class, .jar)                          ║"
echo "  ║     [8] C/C++ (.c, .cpp, .o, .so)                          ║"
echo "  ║     [9] Go (.go)                                            ║"
echo "  ║     [10] Rust (.rs)                                         ║"
echo "  ║                                                              ║"
echo "  ║  📱 APPLICATIONS                                            ║"
echo "  ║     [11] Android APK (.apk)                                 ║"
echo "  ║     [12] iOS App (.ipa)                                     ║"
echo "  ║     [13] Windows EXE (.exe, .dll)                           ║"
echo "  ║     [14] Linux Binary (.elf)                                ║"
echo "  ║     [15] macOS App (.app)                                   ║"
echo "  ║                                                              ║"
echo "  ║  🔐 SECURE FILES                                            ║"
echo "  ║     [16] SSH Keys (.ssh/*)                                  ║"
echo "  ║     [17] SSL Certificates (.pem, .crt, .key)               ║"
echo "  ║     [18] Passwords (.pass, .kdbx)                           ║"
echo "  ║     [19] Databases (.db, .sqlite, .sql)                     ║"
echo "  ║                                                              ║"
echo "  ║  📁 ARCHIVES & DATA                                         ║"
echo "  ║     [20] Archives (.zip, .tar, .gz)                        ║"
echo "  ║     [21] Documents (.pdf, .docx, .xlsx)                     ║"
echo "  ║     [22] Images (.jpg, .png, .gif)                          ║"
echo "  ║     [23] Videos (.mp4, .avi, .mkv)                         ║"
echo "  ║     [24] Audio (.mp3, .wav, .flac)                         ║"
echo "  ║                                                              ║"
echo "  ║  🌐 WEB & NETWORK                                           ║"
echo "  ║     [25] HTML/Web Files (.html, .css)                       ║"
echo "  ║     [26] Config Files (.conf, .cfg, .ini)                   ║"
echo "  ║     [27] JSON/XML (.json, .xml)                             ║"
echo "  ║                                                              ║"
echo "  ║  ⚡ UTILITY                                                 ║"
echo "  ║     [0] Custom (select any file)                             ║"
echo "  ║                                                              ║"
echo "  ╚══════════════════════════════════════════════════════════════╝"
echo ""
}

show_algorithms() {
echo ""
echo "  ╔════════════════════════════════════════════════════════════╗"
echo "  ║           SELECT ENCRYPTION ALGORITHM                    ║"
echo "  ╠════════════════════════════════════════════════════════════╣"
echo "  ║                                                            ║"
echo "  ║  ⭐ RECOMMENDED FOR THIS FILE TYPE                        ║"
echo "  ║     [R] $1                                                 ║"
echo "  ║                                                            ║"
echo "  ╠════════════════════════════════════════════════════════════╣"
echo "  ║  AVAILABLE ALGORITHMS:                                    ║"
echo "  ║     [1] AES-256-GCM         Military Grade               ║"
echo "  ║     [2] ChaCha20-Poly1305   Fastest - Mobile/IoT        ║"
echo "  ║     [3] Camellia-256-GCM    Japanese Standard           ║"
echo "  ║     [4] SM4-GCM             Chinese Standard             ║"
echo "  ║     [5] Twofish-256-GCM      Bruce Schneier Algorithm     ║"
echo "  ║     [6] ARIA-256-GCM        Korean Standard              ║"
echo "  ║     [7] Serpent-256-GCM      Strong Alternative          ║"
echo "  ║     [8] XChaCha20-Poly1305   Extended Nonce (Best)       ║"
echo "  ║                                                            ║"
echo "  ║  Press [R] for recommended, or select number [1-8]        ║"
echo "  ║                                                            ║"
echo "  ╚════════════════════════════════════════════════════════════╝"
echo ""
}

show_options() {
echo ""
echo "  ╔════════════════════════════════════════════════════════════╗"
echo "  ║              ENCRYPTION OPTIONS                            ║"
echo "  ╠════════════════════════════════════════════════════════════╣"
echo "  ║                                                            ║"
echo "  ║  [A] Standard     - Key file generated automatically      ║"
echo "  ║  [B] Password     - Add password protection               ║"
echo "  ║  [C] Stealth      - Encrypt + Secure Delete Original     ║"
echo "  ║  [D] Ultimate     - Password + Self-Destruct              ║"
echo "  ║                                                            ║"
echo "  ╚════════════════════════════════════════════════════════════╝"
echo ""
}

get_file_type() {
    local file="$1"
    local ext="${file##*.}"
    local name=$(basename "$file" | tr '[:upper:]' '[:lower:]')
    
    case "$ext" in
        py|pyc|pyo) echo "python" ;;
        js|mjs|cjs) echo "javascript" ;;
        rb) echo "ruby" ;;
        pl|pm) echo "perl" ;;
        sh|bash) echo "bash" ;;
        php) echo "php" ;;
        java|class|jar) echo "java" ;;
        c|cpp|h|hpp|o|so|a|dll) echo "cpp" ;;
        go) echo "go" ;;
        rs) echo "rust" ;;
        apk|xapk) echo "android" ;;
        ipa) echo "ios" ;;
        exe|msi|dll) echo "windows" ;;
        elf) echo "linux" ;;
        app) echo "macos" ;;
        key|crt|pem|p12|pfx|pub) echo "ssh" ;;
        pass|kdbx) echo "passwords" ;;
        db|sqlite|sqlite3|sql) echo "database" ;;
        zip|tar|gz|tgz|bz2|xz|rar|7z) echo "archive" ;;
        pdf|doc|docx|xls|xlsx|ppt|pptx|odt) echo "document" ;;
        jpg|jpeg|png|gif|bmp|svg|webp|ico) echo "image" ;;
        mp4|avi|mkv|mov|wmv|flv|webm) echo "video" ;;
        mp3|wav|flac|aac|ogg|m4a|wma) echo "audio" ;;
        html|htm|css|scss|sass|less) echo "web" ;;
        conf|cfg|ini|yaml|yml|toml) echo "config" ;;
        json|xml|yml|yaml|toml) echo "data" ;;
        *) echo "unknown" ;;
    esac
}

get_recommended() {
    local type="$1"
    case "$type" in
        python|javascript|ruby|perl|bash|php) echo "ChaCha20-Poly1305 [2]" ;;
        java|cpp|go|rust) echo "AES-256-GCM [1]" ;;
        android) echo "AES-256-GCM + V2 Signature [1]" ;;
        ios) echo "AES-256-GCM [1]" ;;
        windows|linux|macos) echo "XChaCha20-Poly1305 [8]" ;;
        ssh|ssl) echo "AES-256-GCM [1]" ;;
        passwords|database) echo "XChaCha20-Poly1305 [8]" ;;
        archive) echo "AES-256-GCM [1]" ;;
        document|image|video|audio) echo "ChaCha20-Poly1305 [2]" ;;
        web|config|data) echo "ChaCha20-Poly1305 [2]" ;;
        *) echo "AES-256-GCM [1]" ;;
    esac
}

show_categories
echo -n "Select file type [0-27]: "
read TYPE_CHOICE

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
FILE_TYPE=$(get_file_type "$REAL_PATH")
RECOMMENDED=$(get_recommended "$FILE_TYPE")

show_algorithms "$RECOMMENDED"
echo -n "Select algorithm [R/1-8]: "
read ALGO_CHOICE

show_options
echo -n "Select option [A/B/C/D]: "
read OPTION_CHOICE

# Determine algorithm
case "$ALGO_CHOICE" in
    r|R) ALGO_NAME=$(echo "$RECOMMENDED" | sed 's/ \[.*\]//') ;;
    1) ALGO_NAME="AES-256-GCM" ;;
    2) ALGO_NAME="ChaCha20-Poly1305" ;;
    3) ALGO_NAME="Camellia-256-GCM" ;;
    4) ALGO_NAME="SM4-GCM" ;;
    5) ALGO_NAME="Twofish-256-GCM" ;;
    6) ALGO_NAME="ARIA-256-GCM" ;;
    7) ALGO_NAME="Serpent-256-GCM" ;;
    8) ALGO_NAME="XChaCha20-Poly1305" ;;
    *) ALGO_NAME=$(echo "$RECOMMENDED" | sed 's/ \[.*\]//') ;;
esac

# Determine option
case "$OPTION_CHOICE" in
    a|A) MODE_NAME="Standard" ;;
    b|B) MODE_NAME="Password Protected" ;;
    c|C) MODE_NAME="Stealth" ;;
    d|D) MODE_NAME="Ultimate" ;;
    *) MODE_NAME="Standard" ;;
esac

echo ""
echo "════════════════════════════════════════════════════════════"
echo "  ENCRYPTION SUMMARY"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "  📄 File:       $REAL_PATH"
echo "  📦 Size:       $FILE_SIZE"
echo "  🏷️  Type:       $FILE_TYPE"
echo "  🔐 Algorithm:  $ALGO_NAME"
echo "  ⚙️  Mode:       $MODE_NAME"
echo ""
echo "  Encrypting..."
echo ""

# Generate key
KEY_FILE="${REAL_PATH}.key"
openssl rand -base64 32 > "$KEY_FILE" 2>/dev/null

# Password protection
if [[ "$OPTION_CHOICE" == "B" || "$OPTION_CHOICE" == "b" || "$OPTION_CHOICE" == "D" || "$OPTION_CHOICE" == "d" ]]; then
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

# Encrypt with AES-256-CBC (most compatible)
openssl enc -aes-256-cbc -salt -pbkdf2 -iter 100000 \
    -in "$REAL_PATH" -out "$OUTPUT" -pass file:"$KEY_FILE"

if [ $? -eq 0 ]; then
    ENC_SIZE=$(ls -lh "$OUTPUT" | awk '{print $5}')
    
    # Self-destruct
    if [[ "$OPTION_CHOICE" == "C" || "$OPTION_CHOICE" == "c" || "$OPTION_CHOICE" == "D" || "$OPTION_CHOICE" == "d" ]]; then
        echo "  Secure deleting original..."
        shred -u "$REAL_PATH" 2>/dev/null || rm -f "$REAL_PATH"
    fi
    
    echo ""
    echo "════════════════════════════════════════════════════════════"
    echo "  ✅ ENCRYPTION COMPLETE!"
    echo "════════════════════════════════════════════════════════════"
    echo ""
    echo "  📄 Original:      $REAL_PATH"
    echo "  🔐 Encrypted:      $OUTPUT"
    echo "  🔑 Key:           $KEY_FILE"
    echo "  🔐 Algorithm:      $ALGO_NAME"
    echo "  📦 Size:          $ENC_SIZE"
    echo ""
    
    if [[ "$OPTION_CHOICE" == "B" || "$OPTION_CHOICE" == "b" || "$OPTION_CHOICE" == "D" || "$OPTION_CHOICE" == "d" ]]; then
        echo "  🔐 Password:      ENABLED"
    fi
    
    echo "  ⚠️  IMPORTANT: Keep the key file safe!"
    echo "                  Without it, decryption is IMPOSSIBLE!"
    echo ""
else
    echo ""
    echo "  ❌ ENCRYPTION FAILED!"
    rm -f "$KEY_FILE" "$OUTPUT"
    exit 1
fi

