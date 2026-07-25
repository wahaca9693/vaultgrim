//! Encrypt command handler

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;

use crate::cli::Algorithm;
use crate::crypto::{
    kdf::hash_password,
    symmetric::{self, generate_key, EncryptionAlgorithm},
};
use crate::utils::{secure_delete, format_bytes, format_duration};

const FILE_MAGIC: &[u8] = b"TES1"; // Terminal Encryption Suite v1
const VERSION: u8 = 1;

/// Handle encrypt command
pub fn handle_encrypt(
    input: PathBuf,
    output: Option<PathBuf>,
    key: Option<PathBuf>,
    password: bool,
    algorithm: Algorithm,
    secure_delete_flag: bool,
    verbose: bool,
) -> Result<()> {
    let start_time = Instant::now();
    
    println!();
    println!("{}", "🔒 ENCRYPTION".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();

    // Validate input file
    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }
    
    let input_size = fs::metadata(&input)?.len();
    println!("  {:20} {}", "Input:".cyan(), input.display());
    println!("  {:20} {}", "Size:".cyan(), format_bytes(input_size));
    println!("  {:20} {}", "Algorithm:".cyan(), format_algo_name(&algorithm));

    // Determine output file
    let output_path = output.unwrap_or_else(|| {
        input.with_extension(format!("{}.enc", input.extension().unwrap_or_default().to_string_lossy()))
    });
    println!("  {:20} {}", "Output:".cyan(), output_path.display());

    // Get or generate encryption key
    let (symmetric_key, key_file_path): (Vec<u8>, Option<PathBuf>) = if let Some(ref key_path) = key {
        // Load key from file
        let key_data = fs::read(&key_path)?;
        if verbose {
            println!("  {:20} {}", "Key Source:".cyan(), key_path.display());
        }
        (key_data, None)
    } else {
        // Generate new random symmetric key
        let new_key = generate_key();
        // Save key to file
        let key_path = output_path.with_extension("key");
        fs::write(&key_path, &new_key)?;
        if verbose {
            println!("  {:20} {}", "Key saved to:".cyan(), key_path.display());
        }
        (new_key, Some(key_path))
    };

    // Handle password protection
    let password_key = if password {
        let pwd = Password::new()
            .with_prompt("Enter encryption password")
            .interact()?;
        
        let confirm_pwd = Password::new()
            .with_prompt("Confirm password")
            .interact()?;
        
        if pwd != confirm_pwd {
            anyhow::bail!("Passwords do not match!");
        }
        
        println!();
        println!("{}", "  🔐 Password protection enabled".yellow());
        
        let (hash, salt) = hash_password(pwd.as_bytes())?;
        
        // Encrypt symmetric key with password-derived key
        let encrypted_key = symmetric::encrypt_aes256gcm(&hash, &symmetric_key)?;
        
        Some((encrypted_key.ciphertext, encrypted_key.nonce, salt))
    } else {
        None
    };

    // Read input file
    println!();
    println!("  {} Reading file...", "📖".cyan());
    let plaintext = {
        let mut file = File::open(&input)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data
    };

    // Select encryption algorithm
    let (algo, nonce, ciphertext) = match algorithm {
        Algorithm::Aes256Gcm => {
            let encrypted = symmetric::encrypt_aes256gcm(&symmetric_key, &plaintext)?;
            (EncryptionAlgorithm::Aes256Gcm, encrypted.nonce, encrypted.ciphertext)
        }
        Algorithm::ChaCha20Poly1305 => {
            let encrypted = symmetric::encrypt_chacha20poly1305(&symmetric_key, &plaintext)?;
            (EncryptionAlgorithm::ChaCha20Poly1305, encrypted.nonce, encrypted.ciphertext)
        }
    };

    // Build output file format
    println!("  {} Writing encrypted file...", "💾".cyan());
    
    let mut output_file = File::create(&output_path)?;
    
    // Header
    output_file.write_all(FILE_MAGIC)?;              // Magic bytes
    output_file.write_all(&[VERSION])?;               // Version
    output_file.write_all(&[algo_to_u8(algo)])?;      // Algorithm ID
    
    // Flags
    let mut flags: u8 = 0;
    if password_key.is_some() { flags |= 0x01; }
    output_file.write_all(&[flags])?;
    
    // Nonce
    let nonce_len = nonce.len() as u8;
    output_file.write_all(&[nonce_len])?;
    output_file.write_all(&nonce)?;
    
    // Encrypted symmetric key (if password protected)
    if let Some((encrypted_key, key_nonce, salt)) = password_key {
        let key_nonce_len = key_nonce.len() as u8;
        output_file.write_all(&[key_nonce_len])?;
        output_file.write_all(&key_nonce)?;
        output_file.write_all(&(encrypted_key.len() as u32).to_le_bytes())?;
        output_file.write_all(&encrypted_key)?;
        output_file.write_all(&[salt.len() as u8])?;
        output_file.write_all(&salt)?;
    }
    
    // Encrypted data
    output_file.write_all(&(ciphertext.len() as u64).to_le_bytes())?;
    output_file.write_all(&ciphertext)?;
    
    output_file.flush()?;
    drop(output_file);

    // Secure delete original if requested
    if secure_delete_flag {
        println!();
        println!("  {} Securely deleting original file...", "🗑️".red());
        secure_delete::secure_delete(&input)?;
        println!("  {} Original file securely deleted", "✓".green());
    }

    let duration = start_time.elapsed();
    let output_size = fs::metadata(&output_path)?.len();

    // Print report
    println!();
    println!("{}", "═".repeat(50).cyan());
    println!("  {} {}", "ENCRYPTION".green().bold(), "COMPLETE");
    println!("{}", "═".repeat(50).cyan());
    println!();
    println!("  {:20} {}", "Status:".cyan(), "✓ SUCCESS".green());
    println!("  {:20} {}", "Algorithm:".cyan(), algo.as_str());
    println!("  {:20} {}", "Input Size:".cyan(), format_bytes(input_size));
    println!("  {:20} {}", "Output Size:".cyan(), format_bytes(output_size));
    println!("  {:20} {}", "Duration:".cyan(), format_duration(duration.as_secs_f64()));
    
    if let Some(ref key_path) = key {
        println!("  {:20} {}", "Key:".cyan(), key_path.display());
    } else {
        println!("  {}", "⚠️  Store the key file securely!".yellow().bold());
    }

    if password {
        println!();
        println!("  {}", "🔐 Password-protected encryption".yellow());
    }

    println!();

    Ok(())
}

fn format_algo_name(algo: &Algorithm) -> String {
    match algo {
        Algorithm::Aes256Gcm => "AES-256-GCM".to_string(),
        Algorithm::ChaCha20Poly1305 => "ChaCha20-Poly1305".to_string(),
    }
}

fn algo_to_u8(algo: EncryptionAlgorithm) -> u8 {
    match algo {
        EncryptionAlgorithm::Aes256Gcm => 0x01,
        EncryptionAlgorithm::ChaCha20Poly1305 => 0x02,
        EncryptionAlgorithm::XChaCha20Poly1305 => 0x03,
    }
}

fn u8_to_algo(id: u8) -> Option<EncryptionAlgorithm> {
    match id {
        0x01 => Some(EncryptionAlgorithm::Aes256Gcm),
        0x02 => Some(EncryptionAlgorithm::ChaCha20Poly1305),
        0x03 => Some(EncryptionAlgorithm::XChaCha20Poly1305),
        _ => None,
    }
}
