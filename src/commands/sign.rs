//! Sign command handler

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;

use crate::crypto::signatures::{sign, export_signature_base64};

/// Handle sign command
pub fn handle_sign(
    input: PathBuf,
    private_key: PathBuf,
    password: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    println!();
    println!("{}", "✍️  DIGITAL SIGNATURE".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();

    // Validate input file
    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }

    println!("  {:20} {}", "File:".cyan(), input.display());
    println!("  {:20} {}", "Private Key:".cyan(), private_key.display());

    // Read private key
    let mut key_data = fs::read(&private_key)?;
    
    // Decrypt key if password protected
    if password || is_encrypted_key(&key_data) {
        let pwd = Password::new()
            .with_prompt("Enter private key password")
            .interact()?;
        
        // Try to decrypt
        if let Ok(decrypted) = try_decrypt_key(&key_data, pwd.as_bytes()) {
            key_data = decrypted;
        }
    }

    // Read file to sign
    println!();
    println!("  {} Reading file...", "📖".cyan());
    let file_data = {
        let mut file = fs::File::open(&input)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data
    };

    // Create signature
    println!("  {} Creating signature...", "✍️".cyan());
    
    let signature = sign(&file_data, &key_data)
        .map_err(|e| anyhow::anyhow!("Signing failed: {}", e))?;
    
    let signature_base64 = export_signature_base64(&signature);

    // Determine output file
    let output_path = output.unwrap_or_else(|| {
        input.with_extension(format!("{}.sig", input.extension().unwrap_or_default().to_string_lossy()))
    });

    // Write signature file
    let sig_content = format!("-----BEGIN TES SIGNATURE-----\n{}\n-----END TES SIGNATURE-----\n\nFile: {}\nAlgorithm: Ed25519\n",
        signature_base64,
        input.display()
    );
    
    fs::write(&output_path, sig_content)?;

    println!();
    println!("{}", "═".repeat(50).cyan());
    println!("  {} {}", "SIGNATURE".green().bold(), "CREATED");
    println!("{}", "═".repeat(50).cyan());
    println!();
    println!("  {:20} {}", "Status:".cyan(), "✓ SUCCESS".green());
    println!("  {:20} {}", "Algorithm:".cyan(), "Ed25519");
    println!("  {:20} {}", "Signature:".cyan(), output_path.display());
    println!();
    println!("  Signature (Base64):");
    println!("  {}", signature_base64.chars().take(64).collect::<String>());
    println!();

    Ok(())
}

fn is_encrypted_key(data: &[u8]) -> bool {
    // Simple check: encrypted keys have high entropy
    // This is a heuristic - a more robust solution would use a magic header
    if data.len() < 32 {
        return false;
    }
    
    let entropy = calculate_entropy(data);
    entropy > 7.0 // High entropy suggests encryption
}

fn calculate_entropy(data: &[u8]) -> f64 {
    let mut freq = [0u64; 256];
    for &b in data {
        freq[b as usize] += 1;
    }
    
    let len = data.len() as f64;
    let mut entropy = 0.0;
    
    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    
    entropy
}

fn try_decrypt_key(_key_data: &[u8], _password: &[u8]) -> Result<Vec<u8>> {
    // This is a placeholder - real implementation would use proper key format detection
    anyhow::bail!("Key decryption not implemented in this version")
}
