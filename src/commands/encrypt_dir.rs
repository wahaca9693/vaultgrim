//! Directory encryption command

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;
use walkdir::WalkDir;

use crate::cli::Algorithm;
use crate::crypto::{
    kdf::hash_password,
    symmetric::{self, generate_key, EncryptionAlgorithm},
};
use crate::utils::{format_bytes, format_duration};

const ARCHIVE_MAGIC: &[u8] = b"TESA"; // Terminal Encryption Suite Archive v1

/// Handle encrypt-dir command
pub fn handle_encrypt_dir(
    input_dir: PathBuf,
    output: PathBuf,
    key: Option<PathBuf>,
    password: bool,
    algorithm: Algorithm,
    secure_delete_flag: bool,
    _verbose: bool,
) -> Result<()> {
    let start_time = Instant::now();
    
    println!();
    println!("{}", "📁 DIRECTORY ENCRYPTION".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();

    // Validate input directory
    if !input_dir.is_dir() {
        anyhow::bail!("Input is not a directory: {}", input_dir.display());
    }

    println!("  {:20} {}", "Input Directory:".cyan(), input_dir.display());
    println!("  {:20} {}", "Output Archive:".cyan(), output.display());
    println!("  {:20} {}", "Algorithm:".cyan(), format_algo_name(&algorithm));

    // Count files
    let files: Vec<_> = WalkDir::new(&input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    
    let file_count = files.len();
    let total_size: u64 = files.iter()
        .map(|e| fs::metadata(e.path()).map(|m| m.len()).unwrap_or(0))
        .sum();

    println!("  {:20} {} files", "Files:".cyan(), file_count);
    println!("  {:20} {}", "Total Size:".cyan(), format_bytes(total_size));
    println!();

    // Generate or load symmetric key
    let symmetric_key = if let Some(ref key_path) = key {
        fs::read(key_path)?
    } else {
        let new_key = generate_key();
        println!("  {} New encryption key generated", "✓".green());
        new_key
    };

    // Handle password protection
    let password_data = if password {
        let pwd = Password::new()
            .with_prompt("Enter archive password")
            .interact()?;
        
        let confirm_pwd = Password::new()
            .with_prompt("Confirm password")
            .interact()?;
        
        if pwd != confirm_pwd {
            anyhow::bail!("Passwords do not match!");
        }
        
        let (hash, salt) = hash_password(pwd.as_bytes())?;
        let encrypted_key = symmetric::encrypt_aes256gcm(&hash, &symmetric_key)?;
        
        println!("  {} Password protection enabled", "🔐".yellow());
        
        Some((encrypted_key.ciphertext, encrypted_key.nonce, salt))
    } else {
        None
    };

    // Collect file data
    println!();
    println!("  {} Collecting files...", "📂".cyan());
    
    let mut file_entries = Vec::new();
    let file_paths: Vec<_> = files.iter().map(|e| e.path().to_path_buf()).collect();
    for entry in files {
        let path = entry.path();
        let relative_path = path.strip_prefix(&input_dir).unwrap_or(path);
        
        let mut content = Vec::new();
        File::open(path)?.read_to_end(&mut content)?;
        
        file_entries.push((
            relative_path.to_string_lossy().to_string(),
            content,
        ));
    }

    // Serialize file list and contents
    let mut archive_data = Vec::new();
    
    // Write file count
    archive_data.write_all(&(file_entries.len() as u32).to_le_bytes())?;
    
    for (path, content) in &file_entries {
        // Write path length and path
        let path_bytes = path.as_bytes();
        archive_data.write_all(&(path_bytes.len() as u16).to_le_bytes())?;
        archive_data.write_all(path_bytes)?;
        
        // Write content length and content
        archive_data.write_all(&(content.len() as u64).to_le_bytes())?;
        archive_data.extend_from_slice(content);
    }

    // Encrypt archive data
    println!("  {} Encrypting archive...", "🔒".cyan());
    
    let (algo, nonce, ciphertext) = match algorithm {
        Algorithm::Aes256Gcm => {
            let encrypted = symmetric::encrypt_aes256gcm(&symmetric_key, &archive_data)?;
            (EncryptionAlgorithm::Aes256Gcm, encrypted.nonce, encrypted.ciphertext)
        }
        Algorithm::ChaCha20Poly1305 => {
            let encrypted = symmetric::encrypt_chacha20poly1305(&symmetric_key, &archive_data)?;
            (EncryptionAlgorithm::ChaCha20Poly1305, encrypted.nonce, encrypted.ciphertext)
        }
    };

    // Write archive file
    println!("  {} Writing archive...", "💾".cyan());
    
    let mut out_file = File::create(&output)?;
    
    // Header
    out_file.write_all(ARCHIVE_MAGIC)?;
    out_file.write_all(&[1])?; // Version
    out_file.write_all(&[algo_to_u8(algo)])?;
    
    // Flags
    let flags = if password_data.is_some() { 0x01 } else { 0x00 };
    out_file.write_all(&[flags])?;
    
    // Nonce
    out_file.write_all(&[nonce.len() as u8])?;
    out_file.write_all(&nonce)?;
    
    // Password-protected key
    if let Some((encrypted_key, key_nonce, salt)) = password_data {
        out_file.write_all(&[key_nonce.len() as u8])?;
        out_file.write_all(&key_nonce)?;
        out_file.write_all(&(encrypted_key.len() as u32).to_le_bytes())?;
        out_file.write_all(&encrypted_key)?;
        out_file.write_all(&[salt.len() as u8])?;
        out_file.write_all(&salt)?;
    }
    
    // Ciphertext
    out_file.write_all(&(ciphertext.len() as u64).to_le_bytes())?;
    out_file.write_all(&ciphertext)?;
    
    out_file.flush()?;
    drop(out_file);

    // Secure delete original files if requested
    if secure_delete_flag {
        println!();
        println!("  {} Securely deleting original files...", "🗑️".red());
        
        for path in file_paths {
            crate::utils::secure_delete::secure_delete(&path)?;
        }
        
        println!("  {} Original files securely deleted", "✓".green());
    }

    let duration = start_time.elapsed();
    let output_size = fs::metadata(&output)?.len();

    // Print report
    println!();
    println!("{}", "═".repeat(50).cyan());
    println!("  {} {}", "DIRECTORY ENCRYPTION".green().bold(), "COMPLETE");
    println!("{}", "═".repeat(50).cyan());
    println!();
    println!("  {:20} {}", "Status:".cyan(), "✓ SUCCESS".green());
    println!("  {:20} {}", "Algorithm:".cyan(), algo.as_str());
    println!("  {:20} {}", "Files Archived:".cyan(), file_count);
    println!("  {:20} {}", "Total Size:".cyan(), format_bytes(total_size));
    println!("  {:20} {}", "Archive Size:".cyan(), format_bytes(output_size));
    println!("  {:20} {}", "Duration:".cyan(), format_duration(duration.as_secs_f64()));
    
    if key.is_none() {
        println!();
        println!("  {}", "⚠️  Save the key file securely!".yellow().bold());
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
