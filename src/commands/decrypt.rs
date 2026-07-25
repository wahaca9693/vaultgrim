//! Decrypt command handler

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;

use crate::crypto::symmetric::{self, decrypt_aes256gcm, decrypt_chacha20poly1305, EncryptionAlgorithm};
use crate::utils::{format_bytes, format_duration};

const FILE_MAGIC: &[u8] = b"TES1";

/// Handle decrypt command
pub fn handle_decrypt(
    input: PathBuf,
    output: Option<PathBuf>,
    key: Option<PathBuf>,
    password: bool,
    verbose: bool,
) -> Result<()> {
    let start_time = Instant::now();
    
    println!();
    println!("{}", "🔓 DECRYPTION".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();

    // Validate input file
    if !input.exists() {
        anyhow::bail!("Encrypted file not found: {}", input.display());
    }

    let input_size = fs::metadata(&input)?.len();
    println!("  {:20} {}", "Input:".cyan(), input.display());
    println!("  {:20} {}", "Size:".cyan(), format_bytes(input_size));

    // Read encrypted file
    let mut file = File::open(&input)?;
    let mut header = [0u8; 4];
    file.read_exact(&mut header)?;
    
    if &header[0..4] != FILE_MAGIC {
        anyhow::bail!("Invalid file format - not a TES encrypted file");
    }

    let mut version_buf = [0u8; 1];
    file.read_exact(&mut version_buf)?;
    let version = version_buf[0];
    
    let mut algo_buf = [0u8; 1];
    file.read_exact(&mut algo_buf)?;
    let algo_id = algo_buf[0];
    
    let mut flags_buf = [0u8; 1];
    file.read_exact(&mut flags_buf)?;
    let flags = flags_buf[0];
    
    let algo = match algo_id {
        0x01 => EncryptionAlgorithm::Aes256Gcm,
        0x02 => EncryptionAlgorithm::ChaCha20Poly1305,
        0x03 => EncryptionAlgorithm::XChaCha20Poly1305,
        _ => anyhow::bail!("Unknown algorithm ID: {}", algo_id),
    };

    println!("  {:20} {}", "Algorithm:".cyan(), algo.as_str());
    println!("  {:20} {}", "Version:".cyan(), version);

    // Read nonce
    let mut nonce_len_buf = [0u8; 1];
    file.read_exact(&mut nonce_len_buf)?;
    let nonce_len = nonce_len_buf[0] as usize;
    let mut nonce = vec![0u8; nonce_len];
    file.read_exact(&mut nonce)?;

    // Read encrypted symmetric key (if password protected)
    let mut symmetric_key = if flags & 0x01 != 0 {
        let mut key_nonce_len_buf = [0u8; 1];
        file.read_exact(&mut key_nonce_len_buf)?;
        let key_nonce_len = key_nonce_len_buf[0] as usize;
        let mut key_nonce = vec![0u8; key_nonce_len];
        file.read_exact(&mut key_nonce)?;
        
        let mut encrypted_key_len_buf = [0u8; 4];
        file.read_exact(&mut encrypted_key_len_buf)?;
        let encrypted_key_len = u32::from_le_bytes(encrypted_key_len_buf) as usize;
        let mut encrypted_key = vec![0u8; encrypted_key_len];
        file.read_exact(&mut encrypted_key)?;
        
        let mut salt_len_buf = [0u8; 1];
        file.read_exact(&mut salt_len_buf)?;
        let salt_len = salt_len_buf[0] as usize;
        let mut salt = vec![0u8; salt_len];
        file.read_exact(&mut salt)?;
        
        // Prompt for password
        let pwd = if password {
            Password::new()
                .with_prompt("Enter decryption password")
                .interact()?
        } else {
            Password::new()
                .with_prompt("Password-protected file. Enter password:")
                .interact()?
        };
        
        // Derive key from password
        let password_key = crate::crypto::kdf::derive_key_argon2id(pwd.as_bytes(), &salt)?;
        
        // Decrypt symmetric key
        symmetric::decrypt_aes256gcm(&password_key, &key_nonce, &encrypted_key)?
    } else {
        // Key should be provided or we need to extract it
        Vec::new()
    };

    // Read encapsulated key (if present)
    if flags & 0x02 != 0 {
        let mut enc_key_len_buf = [0u8; 4];
        file.read_exact(&mut enc_key_len_buf)?;
        let enc_key_len = u32::from_le_bytes(enc_key_len_buf) as usize;
        let mut enc_key = vec![0u8; enc_key_len];
        file.read_exact(&mut enc_key)?;
        
        // Key is encapsulated - we'll handle this in the hybrid decryption
    }

    // Read encrypted data
    let mut ciphertext_len_buf = [0u8; 8];
    file.read_exact(&mut ciphertext_len_buf)?;
    let ciphertext_len = u64::from_le_bytes(ciphertext_len_buf) as usize;
    
    let mut ciphertext = vec![0u8; ciphertext_len];
    file.read_exact(&mut ciphertext)?;

    // Get or load symmetric key
    if symmetric_key.is_empty() {
        let key_path = if let Some(ref kp) = key {
            kp.clone()
        } else {
            // Try to find key file with same base name
            let key_path = input.with_extension("key");
            if key_path.exists() {
                key_path
            } else {
                anyhow::bail!("Key file required for decryption (expected: {})", key_path.display());
            }
        };
        
        symmetric_key = fs::read(&key_path)?;
        if verbose {
            println!("  {:20} {}", "Key Source:".cyan(), key_path.display());
        }
    }

    // Decrypt
    println!();
    println!("  {} Decrypting...", "🔐".cyan());
    
    let plaintext = match algo {
        EncryptionAlgorithm::Aes256Gcm => {
            decrypt_aes256gcm(&symmetric_key, &nonce, &ciphertext)?
        }
        EncryptionAlgorithm::ChaCha20Poly1305 => {
            decrypt_chacha20poly1305(&symmetric_key, &nonce, &ciphertext)?
        }
        EncryptionAlgorithm::XChaCha20Poly1305 => {
            decrypt_aes256gcm(&symmetric_key, &nonce, &ciphertext)?
        }
    };

    // Determine output file
    let output_path = output.unwrap_or_else(|| {
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        // Remove .enc extension if present
        let stem = stem.strip_suffix(".enc").unwrap_or(&stem);
        input.parent().unwrap_or_else(|| input.as_ref()).join(stem)
    });

    println!("  {:20} {}", "Output:".cyan(), output_path.display());

    // Write decrypted file
    println!("  {} Writing decrypted file...", "💾".cyan());
    let mut out_file = File::create(&output_path)?;
    out_file.write_all(&plaintext)?;
    out_file.flush()?;
    drop(out_file);

    let duration = start_time.elapsed();
    let output_size = fs::metadata(&output_path)?.len();

    // Print report
    println!();
    println!("{}", "═".repeat(50).cyan());
    println!("  {} {}", "DECRYPTION".green().bold(), "COMPLETE");
    println!("{}", "═".repeat(50).cyan());
    println!();
    println!("  {:20} {}", "Status:".cyan(), "✓ SUCCESS".green());
    println!("  {:20} {}", "Algorithm:".cyan(), algo.as_str());
    println!("  {:20} {}", "Input Size:".cyan(), format_bytes(input_size));
    println!("  {:20} {}", "Output Size:".cyan(), format_bytes(output_size));
    println!("  {:20} {}", "Duration:".cyan(), format_duration(duration.as_secs_f64()));

    println!();

    Ok(())
}
