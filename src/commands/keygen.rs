//! Key generation command

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use dialoguer::Password;

use crate::cli::{ExportFormat, KeyType};
use crate::crypto::{
    asymmetric::{self, export_public_key},
    signatures::{export_public_key_pem as export_sig_pubkey, generate_ed25519_keypair},
    kdf::hash_password,
};

const KEYSTORE_DIR: &str = ".tes/keys";

/// Handle key generation command
pub fn handle_keygen(
    key_type: KeyType,
    output: Option<PathBuf>,
    password: bool,
    show_public: bool,
) -> Result<()> {
    println!();
    println!("{}", "🔑 KEY GENERATION".cyan().bold());
    println!("{}", "═".repeat(40).cyan());
    println!();

    match key_type {
        KeyType::X25519 => generate_x25519(output, password, show_public)?,
        KeyType::X25519Kyber => generate_x25519_kyber(output, password, show_public)?,
        KeyType::Ed25519 => generate_ed25519(output, password, show_public)?,
        KeyType::Rsa4096 => {
            println!("{}", "⚠️  RSA-4096 is deprecated and not recommended for new keys".yellow());
            println!("{}", "   Use Ed25519 or X25519 instead.".yellow());
            println!();
            generate_rsa4096(output, password, show_public)?;
        }
    }

    Ok(())
}

fn generate_x25519(output: Option<PathBuf>, with_password: bool, show_public: bool) -> Result<()> {
    println!("Generating X25519 key pair...");
    
    let keypair = asymmetric::generate_x25519_keypair()
        .map_err(|e| anyhow::anyhow!("Failed to generate X25519 key: {}", e))?;
    
    let (private_key_data, _password_hash, _salt) = if with_password {
        let pwd = Password::new()
            .with_prompt("Enter password for private key")
            .interact()?;
        
        let confirm_pwd = Password::new()
            .with_prompt("Confirm password")
            .interact()?;
        
        if pwd != confirm_pwd {
            anyhow::bail!("Passwords do not match!");
        }
        
        let (hash, salt) = hash_password(pwd.as_bytes())?;
        let encrypted_key = xor_encrypt(&keypair.secret_key, &hash);
        (encrypted_key, Some(hash), Some(salt))
    } else {
        (keypair.secret_key.clone(), None, None)
    };
    
    let output_path = output.unwrap_or_else(|| PathBuf::from("x25519_private.key"));
    fs::write(&output_path, &private_key_data)?;
    
    println!();
    println!("{} Private key saved to: {}", "✓".green(), output_path.display());
    
    if show_public {
        let public_pem = export_public_key(&keypair.public_key, "X25519");
        println!();
        println!("{}", "Public Key:".cyan());
        println!("{}", public_pem);
    }
    
    // Also save public key separately in PEM format
    let public_path = output_path.with_extension("pub.key");
    let public_pem = export_public_key(&keypair.public_key, "X25519");
    fs::write(&public_path, public_pem.as_bytes())?;
    println!("  Public key saved to: {}", public_path.display());
    
    Ok(())
}

fn generate_x25519_kyber(output: Option<PathBuf>, with_password: bool, show_public: bool) -> Result<()> {
    println!("Generating X25519 + Kyber-768 hybrid key pair...");
    println!("{}", "  (Post-Quantum resistant)".yellow());
    
    #[cfg(feature = "pqc")]
    {
        use crate::crypto::asymmetric::generate_hybrid_keypair;
        
        let keypair = generate_hybrid_keypair()
            .map_err(|e| anyhow::anyhow!("Failed to generate hybrid key: {}", e))?;
        
        // Combine keys for storage
        let mut combined_secret = keypair.x25519.secret_key.clone();
        combined_secret.extend_from_slice(&keypair.kyber_secret);
        
        let (private_key_data, _, _) = if with_password {
            let pwd = Password::new()
                .with_prompt("Enter password for private key")
                .interact()?;
            
            let confirm_pwd = Password::new()
                .with_prompt("Confirm password")
                .interact()?;
            
            if pwd != confirm_pwd {
                anyhow::bail!("Passwords do not match!");
            }
            
            let (hash, salt) = hash_password(pwd.as_bytes())?;
            let encrypted_key = xor_encrypt(&combined_secret, &hash);
            (encrypted_key, Some(hash), Some(salt))
        } else {
            (combined_secret, None, None)
        };
        
        let output_path = output.unwrap_or_else(|| PathBuf::from("x25519-kyber_private.key"));
        fs::write(&output_path, &private_key_data)?;
        
        println!();
        println!("{} Private key saved to: {}", "✓".green(), output_path.display());
        
        if show_public {
            let x25519_pub_pem = export_public_key(&keypair.x25519.public_key, "X25519");
            println!();
            println!("{}", "X25519 Public Key:".cyan());
            println!("{}", x25519_pub_pem);
        }
        
        // Save public keys
        let mut combined_public = keypair.x25519.public_key.clone();
        combined_public.extend_from_slice(&keypair.kyber_public);
        
        let public_path = output_path.with_extension("pub.key");
        fs::write(&public_path, combined_public)?;
        println!("  Public key saved to: {}", public_path.display());
    }
    
    #[cfg(not(feature = "pqc"))]
    {
        println!("{}", "⚠️  Kyber support not compiled. Install with --features pqc".yellow());
        println!("  Generating X25519-only key instead...");
        generate_x25519(output, with_password, show_public)?;
    }
    
    Ok(())
}

fn generate_ed25519(output: Option<PathBuf>, with_password: bool, show_public: bool) -> Result<()> {
    println!("Generating Ed25519 signing key pair...");
    
    let keypair = generate_ed25519_keypair()
        .map_err(|e| anyhow::anyhow!("Failed to generate Ed25519 key: {}", e))?;
    
    let (private_key_data, _, _) = if with_password {
        let pwd = Password::new()
            .with_prompt("Enter password for private key")
            .interact()?;
        
        let confirm_pwd = Password::new()
            .with_prompt("Confirm password")
            .interact()?;
        
        if pwd != confirm_pwd {
            anyhow::bail!("Passwords do not match!");
        }
        
        let (hash, salt) = hash_password(pwd.as_bytes())?;
        let encrypted_key = xor_encrypt(&keypair.secret_key, &hash);
        (encrypted_key, Some(hash), Some(salt))
    } else {
        (keypair.secret_key.clone(), None, None)
    };
    
    let output_path = output.unwrap_or_else(|| PathBuf::from("ed25519_private.key"));
    fs::write(&output_path, &private_key_data)?;
    
    println!();
    println!("{} Private key saved to: {}", "✓".green(), output_path.display());
    
    if show_public {
        let public_pem = export_sig_pubkey(&keypair.public_key);
        println!();
        println!("{}", "Public Key:".cyan());
        println!("{}", public_pem);
    }
    
    let public_path = output_path.with_extension("pub.key");
    let public_pem = export_sig_pubkey(&keypair.public_key);
    fs::write(&public_path, public_pem.as_bytes())?;
    println!("  Public key saved to: {}", public_path.display());
    
    Ok(())
}

fn generate_rsa4096(output: Option<PathBuf>, with_password: bool, show_public: bool) -> Result<()> {
    println!("Generating RSA-4096 key pair...");
    println!("{}", "  (This may take a while...)".yellow());
    
    use rsa::RsaPrivateKey;
    use rsa::pkcs8::{EncodePrivateKey, LineEnding};
    
    let mut rng = rand::rngs::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 4096)
        .map_err(|e| anyhow::anyhow!("RSA key generation failed: {}", e))?;
    
    let private_pem = private_key.to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| anyhow::anyhow!("Failed to encode private key: {}", e))?;
    
    let (private_key_data, _, _) = if with_password {
        let pwd = Password::new()
            .with_prompt("Enter password for private key")
            .interact()?;
        
        let confirm_pwd = Password::new()
            .with_prompt("Confirm password")
            .interact()?;
        
        if pwd != confirm_pwd {
            anyhow::bail!("Passwords do not match!");
        }
        
        let (hash, salt) = hash_password(pwd.as_bytes())?;
        let encrypted_key = xor_encrypt(private_pem.as_bytes(), &hash);
        (encrypted_key, Some(hash), Some(salt))
    } else {
        (private_pem.as_bytes().to_vec(), None, None)
    };
    
    let output_path = output.unwrap_or_else(|| PathBuf::from("rsa4096_private.pem"));
    fs::write(&output_path, &private_key_data)?;
    
    println!();
    println!("{} Private key saved to: {}", "✓".green(), output_path.display());
    
    if show_public {
        let public_key = private_key.to_public_key();
        println!();
        println!("{}", "Public Key:".cyan());
        println!("{:?}", public_key);
    }
    
    Ok(())
}

fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

/// Handle key import command
pub fn handle_import(
    input: PathBuf,
    _key_type: Option<KeyType>,
    output: Option<String>,
    _password: bool,
) -> Result<()> {
    println!();
    println!("{}", "📥 KEY IMPORT".cyan().bold());
    println!("{}", "═".repeat(40).cyan());
    println!();
    
    let key_data = fs::read(&input)
        .map_err(|e| anyhow::anyhow!("Failed to read key file: {}", e))?;
    
    let key_name = output.unwrap_or_else(|| {
        input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("imported_key")
            .to_string()
    });
    
    // Create key store directory
    let store_dir = PathBuf::from(KEYSTORE_DIR);
    fs::create_dir_all(&store_dir)?;
    
    // Save to key store
    let key_path = store_dir.join(format!("{}.key", key_name));
    fs::write(&key_path, &key_data)?;
    
    println!("{} Key imported successfully!", "✓".green());
    println!("  Key ID: {}", key_name);
    println!("  Saved to: {}", key_path.display());
    
    Ok(())
}

/// Handle key export command
pub fn handle_export(
    key_id: String,
    output: PathBuf,
    _password: bool,
    format: ExportFormat,
) -> Result<()> {
    println!();
    println!("{}", "📤 KEY EXPORT".cyan().bold());
    println!("{}", "═".repeat(40).cyan());
    println!();
    
    let store_dir = PathBuf::from(KEYSTORE_DIR);
    let key_path = store_dir.join(format!("{}.key", key_id));
    
    if !key_path.exists() {
        anyhow::bail!("Key not found: {}", key_id);
    }
    
    let key_data = fs::read(&key_path)?;
    
    let exported = match format {
        ExportFormat::Pem => {
            use base64::{engine::general_purpose::STANDARD, Engine};
            format!(
                "-----BEGIN TES KEY-----\n{}\n-----END TES KEY-----",
                STANDARD.encode(&key_data)
            )
        }
        ExportFormat::Base64 => {
            use base64::{engine::general_purpose::STANDARD, Engine};
            STANDARD.encode(&key_data)
        }
        ExportFormat::Raw => {
            String::from_utf8(key_data)?
        }
    };
    
    fs::write(&output, exported.as_bytes())?;
    
    println!("{} Key exported successfully!", "✓".green());
    println!("  Key ID: {}", key_id);
    println!("  Exported to: {}", output.display());
    
    Ok(())
}

/// Handle key list command
pub fn handle_list() -> Result<()> {
    println!();
    println!("{}", "📋 STORED KEYS".cyan().bold());
    println!("{}", "═".repeat(40).cyan());
    println!();
    
    let store_dir = PathBuf::from(KEYSTORE_DIR);
    
    if !store_dir.exists() {
        println!("{}", "No keys stored yet.".yellow());
        println!("  Use `tes keygen` to generate new keys.");
        return Ok(());
    }
    
    let entries: Vec<_> = fs::read_dir(&store_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "key").unwrap_or(false))
        .collect();
    
    let total_keys = entries.len();
    if total_keys == 0 {
        println!("{}", "No keys stored yet.".yellow());
        return Ok(());
    }
    
    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let key_id = name_str.strip_suffix(".key").unwrap_or(&name_str);
        
        let metadata = fs::metadata(entry.path())?;
        let size = metadata.len();
        
        println!("  {:30} {} bytes", key_id.cyan(), size);
    }
    
    println!();
    println!("  Total: {} keys", total_keys);
    
    Ok(())
}

/// Handle change password command
pub fn handle_change_password(
    key_id: String,
    _old_password: bool,
    _new_password: bool,
) -> Result<()> {
    println!();
    println!("{}", "🔄 CHANGE KEY PASSWORD".cyan().bold());
    println!("{}", "═".repeat(40).cyan());
    println!();
    
    let store_dir = PathBuf::from(KEYSTORE_DIR);
    let key_path = store_dir.join(format!("{}.key", key_id));
    
    if !key_path.exists() {
        anyhow::bail!("Key not found: {}", key_id);
    }
    
    println!("Password change functionality - use keygen to regenerate if needed");
    
    Ok(())
}
