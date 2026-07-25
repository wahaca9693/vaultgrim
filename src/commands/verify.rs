//! Verify command handler

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::crypto::signatures::{verify, import_signature_base64, import_public_key_pem};

/// Handle verify command
pub fn handle_verify(
    input: PathBuf,
    signature: PathBuf,
    public_key: PathBuf,
) -> Result<()> {
    println!();
    println!("{}", "✅ SIGNATURE VERIFICATION".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();

    // Validate files
    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }
    if !signature.exists() {
        anyhow::bail!("Signature file not found: {}", signature.display());
    }
    if !public_key.exists() {
        anyhow::bail!("Public key file not found: {}", public_key.display());
    }

    println!("  {:20} {}", "File:".cyan(), input.display());
    println!("  {:20} {}", "Signature:".cyan(), signature.display());
    println!("  {:20} {}", "Public Key:".cyan(), public_key.display());

    // Read file
    println!();
    println!("  {} Reading file...", "📖".cyan());
    let file_data = {
        let mut file = fs::File::open(&input)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data
    };

    // Read signature
    println!("  {} Reading signature...", "📜".cyan());
    let sig_content = fs::read_to_string(&signature)?;
    let signature_base64 = sig_content
        .lines()
        .filter(|l| !l.starts_with("-----") && !l.starts_with("File:") && !l.starts_with("Algorithm:") && !l.trim().is_empty())
        .collect::<String>();
    
    let signature = import_signature_base64(&signature_base64)
        .map_err(|e| anyhow::anyhow!("Invalid signature format: {}", e))?;

    // Read public key
    println!("  {} Reading public key...", "🔑".cyan());
    let pubkey_content = fs::read_to_string(&public_key)?;
    let public_key = import_public_key_pem(&pubkey_content)
        .or_else(|_| {
            // Try raw format
            fs::read(&public_key).map_err(|e| format!("IO error: {}", e))
        })
        .map_err(|e| anyhow::anyhow!("Invalid public key format: {}", e))?;

    // Verify signature
    println!("  {} Verifying signature...", "🔍".cyan());
    
    let is_valid = verify(&file_data, &signature, &public_key)
        .map_err(|e| anyhow::anyhow!("Verification failed: {}", e))?;

    // Print result
    println!();
    println!("{}", "═".repeat(50).cyan());
    
    if is_valid {
        println!("  {}", "✅ SIGNATURE VERIFIED".green().bold());
        println!();
        println!("  {:20} {}", "Status:".cyan(), "✓ VALID".green());
        println!("  {:20} {}", "Algorithm:".cyan(), "Ed25519");
        println!("  {:20} {}", "File:".cyan(), input.display());
    } else {
        println!("  {}", "❌ SIGNATURE INVALID".red().bold());
        println!();
        println!("  {:20} {}", "Status:".cyan(), "✗ INVALID".red());
        println!("  {:20} {}", "Algorithm:".cyan(), "Ed25519");
        println!();
        println!("  {}", "⚠️  WARNING: The file may have been tampered with!".yellow().bold());
        
        // Return error for invalid signature
        println!();
        println!("{}", "═".repeat(50).cyan());
        println!();
        
        anyhow::bail!("Signature verification failed - file may have been tampered with!");
    }
    
    println!();
    println!("{}", "═".repeat(50).cyan());
    println!();

    Ok(())
}
