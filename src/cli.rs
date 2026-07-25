//! CLI argument definitions using clap

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "vaultgrim",
    author = "Vaultgrim Team",
    version,
    about = "🔐 Vaultgrim - Industry-standard terminal encryption with Post-Quantum resistance",
    long_about = None,
    next_help_heading = "Commands",
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 🔒 Encrypt a file
    Encrypt {
        /// Input file to encrypt
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Output file (default: FILE.ext.enc)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        
        /// Key file (default: generate random key)
        #[arg(short, long, value_name = "FILE")]
        key: Option<PathBuf>,
        
        /// Add password protection (prompts for password)
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Encryption algorithm
        #[arg(short, long, value_enum, default_value = "aes-256-gcm")]
        algorithm: Algorithm,
        
        /// Securely delete original file after encryption
        #[arg(long, action = clap::ArgAction::SetTrue)]
        secure_delete: bool,
        
        /// Verbose output
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
    
    /// 🔓 Decrypt a file
    Decrypt {
        /// Encrypted file to decrypt
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Output file (default: FILE.decrypted)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        
        /// Key file (required if key was not stored)
        #[arg(short, long, value_name = "FILE")]
        key: Option<PathBuf>,
        
        /// Password for key decryption (prompts if needed)
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Verbose output
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
    
    /// 📁 Encrypt a directory recursively
    EncryptDir {
        /// Input directory to encrypt
        #[arg(value_name = "DIR")]
        input_dir: PathBuf,
        
        /// Output archive file
        #[arg(short, long, value_name = "FILE", required = true)]
        output: PathBuf,
        
        /// Key file (default: generate random key)
        #[arg(short, long, value_name = "FILE")]
        key: Option<PathBuf>,
        
        /// Add password protection
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Encryption algorithm
        #[arg(short, long, value_enum, default_value = "aes-256-gcm")]
        algorithm: Algorithm,
        
        /// Securely delete original files after encryption
        #[arg(long, action = clap::ArgAction::SetTrue)]
        secure_delete: bool,
        
        /// Verbose output
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
    
    /// 🔑 Generate cryptographic keys
    Keygen {
        /// Key type to generate
        #[arg(value_enum, default_value = "x25519-kyber")]
        key_type: KeyType,
        
        /// Output file for the private key
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        
        /// Password to encrypt the private key
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Show public key in output
        #[arg(long, action = clap::ArgAction::SetTrue)]
        show_public: bool,
    },
    
    /// ✍️ Sign a file
    Sign {
        /// File to sign
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Private key file
        #[arg(short, long, value_name = "FILE", required = true)]
        private_key: PathBuf,
        
        /// Password for private key (prompts if needed)
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Output signature file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    
    /// ✅ Verify a signature
    Verify {
        /// File that was signed
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Signature file
        #[arg(short, long, value_name = "FILE", required = true)]
        signature: PathBuf,
        
        /// Public key file
        #[arg(short, long, value_name = "FILE", required = true)]
        public_key: PathBuf,
    },
    
    /// 📥 Import a key
    KeyImport {
        /// Key file to import
        #[arg(value_name = "FILE")]
        input: PathBuf,
        
        /// Key type (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        key_type: Option<KeyType>,
        
        /// Output name in key store
        #[arg(short, long, value_name = "NAME")]
        output: Option<String>,
        
        /// Password if key is encrypted
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
    },
    
    /// 📤 Export a key
    KeyExport {
        /// Key ID to export
        #[arg(value_name = "KEY_ID")]
        key_id: String,
        
        /// Output file
        #[arg(short, long, value_name = "FILE", required = true)]
        output: PathBuf,
        
        /// Password for key file
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        password: bool,
        
        /// Export format
        #[arg(short, long, value_enum, default_value = "pem")]
        format: ExportFormat,
    },
    
    /// 📋 List stored keys
    KeyList {
        /// Show key details (not just IDs)
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        detailed: bool,
    },
    
    /// 🔄 Change key password
    KeyPassword {
        /// Key ID to change password
        #[arg(value_name = "KEY_ID")]
        key_id: String,
        
        /// Current password
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        old_password: bool,
        
        /// New password
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        new_password: bool,
    },
    
    /// ⚡ Run cryptographic benchmark
    Benchmark,

    /// 🎯 Interactive menu (select options by number)
    Menu,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum Algorithm {
    /// AES-256-GCM (default, recommended)
    #[value(name = "aes-256-gcm")]
    Aes256Gcm,
    
    /// ChaCha20-Poly1305 (good for mobile/IoT)
    #[value(name = "chacha20-poly1305")]
    ChaCha20Poly1305,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum KeyType {
    /// X25519 key exchange (default)
    #[value(name = "x25519")]
    X25519,
    
    /// X25519 + Kyber-768 hybrid (Post-Quantum)
    #[value(name = "x25519-kyber")]
    X25519Kyber,
    
    /// Ed25519 signing key
    #[value(name = "ed25519")]
    Ed25519,
    
    /// RSA-4096 key (legacy compatibility)
    #[value(name = "rsa:4096")]
    Rsa4096,
}

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum ExportFormat {
    /// PEM format (default)
    #[value(name = "pem")]
    Pem,
    
    /// Base64 encoded
    #[value(name = "base64")]
    Base64,
    
    /// Raw binary
    #[value(name = "raw")]
    Raw,
}
