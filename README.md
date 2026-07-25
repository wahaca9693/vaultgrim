# 🔐 Terminal Encryption Suite (TES)

أداة تشفير طرفية (CLI) تجمع أقوى أنظمة التشفير المعتمدة عالمياً مع واجهة استخدام بسيطة وسريعة.

A terminal encryption tool with industry-standard cryptographic primitives, Post-Quantum resistance support, and a simple CLI interface.

## ✨ Features | المميزات

### Encryption Algorithms | خوارزميات التشفير
- **AES-256-GCM** - Industry standard symmetric encryption
- **ChaCha20-Poly1305** - Great for mobile/IoT devices
- **X25519+Kyber** - Hybrid Post-Quantum resistant encryption

### Key Exchange & Signatures | تبادل المفاتيح والتوقيعات
- **X25519** - Modern elliptic curve key exchange
- **Ed25519** - Fast and secure digital signatures
- **RSA-4096** - Legacy compatibility (deprecated)

### Key Derivation | اشتقاق المفاتيح
- **Argon2id** - Memory-hard password-based key derivation
- **HKDF** - HMAC-based key derivation

### Security Features | ميزات الأمان
- Secure file deletion with multi-pass overwrite
- Password-protected encryption layer
- Random nonce/IV for every encryption operation
- No custom crypto - only audited, proven libraries

## 📦 Installation | التثبيت

### Build from Source | البناء من الكود المصدري

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/example/terminal-encryption-suite.git
cd terminal-encryption-suite
cargo build --release

# Binary will be at target/release/terminal-encryption-suite
```

### Pre-built Binaries | النسخ التنفيذية الجاهزة
Check the releases page for pre-built binaries for:
- Linux (x86_64, ARM)
- macOS
- Windows
- Termux (Android)

## 🚀 Quick Start | البدء السريع

### Encrypt a File | تشفير ملف
```bash
# Basic encryption (generates a random key)
tes encrypt myfile.txt

# With password protection
tes encrypt myfile.txt --password

# With specific algorithm
tes encrypt myfile.txt --algorithm chacha20-poly1305

# With secure delete (removes original after encryption)
tes encrypt myfile.txt --secure-delete
```

### Decrypt a File | فك تشفير ملف
```bash
# Decrypt (automatically finds .key file)
tes decrypt myfile.txt.enc

# With custom key location
tes decrypt myfile.txt.enc --key mykey.key
```

### Generate Keys | توليد المفاتيح
```bash
# X25519 key pair (default)
tes keygen

# Ed25519 signing key
tes keygen ed25519 --show-public

# RSA-4096 (deprecated)
tes keygen rsa:4096
```

### Sign and Verify | توقيع والتحقق
```bash
# Sign a file
tes sign document.pdf --private-key mykey.key

# Verify signature
tes verify document.pdf --signature document.sig --public-key mykey.pub
```

### Encrypt Directories | تشفير المجلدات
```bash
tes encrypt-dir ./my_folder/ -o backup.tar.enc
```

## 📖 CLI Reference | مرجع الأوامر

### Global Options | الخيارات العامة
```
-h, --help     Show help
-V, --version  Show version
```

### Commands | الأوامر

#### encrypt | التشفير
```
tes encrypt <FILE> [OPTIONS]

Options:
  -o, --output <FILE>      Output file
  -k, --key <FILE>         Key file (default: generate new)
  -p, --password           Add password protection
  -a, --algorithm <ALGO>  Algorithm: aes-256-gcm, chacha20-poly1305, x25519-kyber
  --secure-delete          Securely delete original after encryption
  -v, --verbose            Verbose output
```

#### decrypt | فك التشفير
```
tes decrypt <FILE> [OPTIONS]

Options:
  -o, --output <FILE>      Output file
  -k, --key <FILE>         Key file
  -p, --password           Password-protected key
  -v, --verbose            Verbose output
```

#### keygen | توليد المفاتيح
```
tes keygen [KEY_TYPE] [OPTIONS]

Key Types:
  x25519              X25519 key exchange (default)
  x25519-kyber        X25519 + Kyber-768 hybrid (Post-Quantum)
  ed25519             Ed25519 signing key
  rsa:4096            RSA-4096 (deprecated)

Options:
  -o, --output <FILE>     Output file
  -p, --password          Password-protect private key
  --show-public            Display public key
```

#### encrypt-dir | تشفير مجلد
```
tes encrypt-dir <DIR> -o <OUTPUT> [OPTIONS]

Options:
  -o, --output <FILE>      Output archive file
  -k, --key <FILE>         Key file
  -p, --password           Password protection
  -a, --algorithm <ALGO>  Algorithm
  --secure-delete          Delete originals after encryption
```

#### sign | توقيع
```
tes sign <FILE> -k <KEY> [OPTIONS]

Options:
  -k, --private-key <FILE>  Private key file (required)
  -p, --password            Key password
  -o, --output <FILE>       Signature output file
```

#### verify | التحقق
```
tes verify <FILE> -s <SIG> -k <KEY>

Options:
  -s, --signature <FILE>   Signature file (required)
  -k, --public-key <FILE>  Public key file (required)
```

#### benchmark | اختبار الأداء
```
tes benchmark
```
Runs cryptographic performance tests on your system.

## 🔒 Security Notes | ملاحظات الأمان

1. **Key Management | إدارة المفاتيح**
   - Always store keys securely
   - Never share private keys
   - Use password protection for sensitive keys

2. **Password Protection | الحماية بكلمة سر**
   - Use strong passwords (minimum 12 characters)
   - Passwords are used with Argon2id for key derivation

3. **Secure Delete | الحذف الآمن**
   - The `--secure-delete` option overwrites files 3 times before deletion
   - Still not guarantees against data recovery from SSDs with wear leveling

4. **Algorithm Selection | اختيار الخوارزمية**
   - AES-256-GCM: Best for most use cases
   - ChaCha20-Poly1305: Better for mobile/IoT devices
   - X25519-Kyber: Future-proof against quantum computers

## 📊 Performance | الأداء

Typical performance on modern hardware (10MB file):

| Algorithm | Encryption | Decryption | Throughput |
|-----------|-----------|------------|------------|
| AES-256-GCM | ~13ms | ~13ms | ~755 MB/s |
| ChaCha20-Poly1305 | ~11ms | ~12ms | ~885 MB/s |

## 🔧 Building with Post-Quantum Support | البناء مع دعم ما بعد الكم

To enable Kyber-768 hybrid encryption (requires libclang):

```bash
cargo build --release --features pqc
```

## 📁 File Format | صيغة الملفات

Encrypted files use a custom format:
```
┌──────────────────────────────────────┐
│ MAGIC (TES1)    │ VERSION │ ALGO     │
├──────────────────────────────────────┤
│ FLAGS │ NONCE_LEN │ NONCE            │
├──────────────────────────────────────┤
│ [OPTIONAL] Encrypted Key + Salt       │
├──────────────────────────────────────┤
│ [OPTIONAL] Encapsulated Key           │
├──────────────────────────────────────┤
│ Ciphertext Length │ Ciphertext        │
└──────────────────────────────────────┘
```

## 🤝 Contributing | المساهمة

Contributions welcome! Please read the security guidelines before submitting changes.

## 📄 License | الترخيص

MIT License or Apache 2.0

## ⚠️ Disclaimer | إخلاء المسؤولية

This software is provided "as is" without warranty. Always verify encryption/decryption correctness for critical use cases.
