//! Master Password Keystore
//! 
//! Encrypted key storage with master password protection.
//! Keys are stored in a file encrypted with AES-256-GCM,
//! with the encryption key derived from the master password via Argon2id.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::crypto::kdf::{derive_key_argon2id, generate_salt};
use crate::crypto::symmetric::{decrypt_aes256gcm, encrypt_aes256gcm};

const KEYSTORE_MAGIC: &[u8] = b"TESK"; // Terminal Encryption Suite Keystore
const KEYSTORE_VERSION: u8 = 1;

/// Master password keystore
#[derive(Debug, Serialize, Deserialize)]
pub struct Keystore {
    /// Salt for Argon2id
    pub salt: Vec<u8>,
    /// Encrypted keys (ciphertext)
    pub encrypted_keys: Vec<u8>,
    /// Nonce for AES-256-GCM
    pub nonce: Vec<u8>,
}

/// Initialize a new keystore with master password
pub fn create_keystore(master_password: &str) -> Result<(Keystore, Vec<u8>), String> {
    // Generate salt for Argon2id
    let salt = generate_salt();
    
    // Derive key from master password
    let master_key = derive_key_argon2id(master_password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;
    
    // Create empty keystore data
    let keys_data = KeystoreKeys {
        keys: Vec::new(),
    };
    let keys_json = serde_json::to_vec(&keys_data).map_err(|e| e.to_string())?;
    
    // Encrypt the keys data
    let encrypted = encrypt_aes256gcm(&master_key, &keys_json)
        .map_err(|e| e.to_string())?;
    
    let keystore = Keystore {
        salt,
        encrypted_keys: encrypted.ciphertext,
        nonce: encrypted.nonce,
    };
    
    Ok((keystore, master_key))
}

/// Open an existing keystore with master password
pub fn open_keystore(master_password: &str, keystore: &Keystore) -> Result<Vec<u8>, String> {
    // Derive key from master password
    let master_key = derive_key_argon2id(master_password.as_bytes(), &keystore.salt)
        .map_err(|e| e.to_string())?;
    
    // Decrypt the keys data
    let keys_json = decrypt_aes256gcm(&master_key, &keystore.nonce, &keystore.encrypted_keys)
        .map_err(|_| "Invalid master password or corrupted keystore".to_string())?;
    
    Ok(master_key)
}

/// Save keystore to file
pub fn save_keystore(keystore: &Keystore, path: &PathBuf) -> Result<(), String> {
    let data = serde_json::to_vec(keystore).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())?;
    Ok(())
}

/// Load keystore from file
pub fn load_keystore(path: &PathBuf) -> Result<Keystore, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let keystore: Keystore = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
    
    // Verify magic bytes
    // (In a real implementation, we'd verify the magic here)
    
    Ok(keystore)
}

/// Check if keystore exists
pub fn keystore_exists(path: &PathBuf) -> bool {
    path.exists()
}

/// Update keystore with new keys
pub fn update_keystore(
    master_password: &str,
    keystore: &Keystore,
    new_keys: Vec<StoredKey>,
) -> Result<Keystore, String> {
    // Derive key from master password
    let master_key = derive_key_argon2id(master_password.as_bytes(), &keystore.salt)
        .map_err(|e| e.to_string())?;
    
    // Decrypt existing keys
    let keys_json = decrypt_aes256gcm(&master_key, &keystore.nonce, &keystore.encrypted_keys)
        .map_err(|_| "Invalid master password or corrupted keystore".to_string())?;
    
    let mut keys: KeystoreKeys = serde_json::from_slice(&keys_json).map_err(|e| e.to_string())?;
    
    // Add new keys
    keys.keys.extend(new_keys);
    
    let updated_json = serde_json::to_vec(&keys).map_err(|e| e.to_string())?;
    
    // Re-encrypt
    let encrypted = encrypt_aes256gcm(&master_key, &updated_json)
        .map_err(|e| e.to_string())?;
    
    Ok(Keystore {
        salt: keystore.salt.clone(),
        encrypted_keys: encrypted.ciphertext,
        nonce: encrypted.nonce,
    })
}

/// Get keys from keystore
pub fn get_keys(master_password: &str, keystore: &Keystore) -> Result<Vec<StoredKey>, String> {
    // Derive key from master password
    let _master_key = derive_key_argon2id(master_password.as_bytes(), &keystore.salt)
        .map_err(|e| e.to_string())?;
    
    // Decrypt keys data
    let keys_json = decrypt_aes256gcm(&_master_key, &keystore.nonce, &keystore.encrypted_keys)
        .map_err(|_| "Invalid master password or corrupted keystore".to_string())?;
    
    let keys: KeystoreKeys = serde_json::from_slice(&keys_json).map_err(|e| e.to_string())?;
    
    Ok(keys.keys)
}

/// Stored key entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredKey {
    /// Unique key ID
    pub id: String,
    /// Key type (x25519, ed25519, etc.)
    pub key_type: String,
    /// Encrypted private key data
    pub encrypted_key: Vec<u8>,
    /// Public key data
    pub public_key: Vec<u8>,
    /// Key nonce for AES-256-GCM
    pub nonce: Vec<u8>,
    /// Salt for Argon2id (to derive key from password if needed)
    pub salt: Vec<u8>,
    /// Creation timestamp
    pub created: String,
    /// Optional description
    pub description: Option<String>,
}

/// Keys container
#[derive(Debug, Serialize, Deserialize)]
struct KeystoreKeys {
    keys: Vec<StoredKey>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_open_keystore() {
        let password = "test_master_password_123";
        
        // Create keystore
        let (keystore, master_key) = create_keystore(password).unwrap();
        
        // Master key should be 32 bytes
        assert_eq!(master_key.len(), 32);
        
        // Keystore should have data
        assert!(!keystore.encrypted_keys.is_empty());
        assert!(!keystore.nonce.is_empty());
        assert!(!keystore.salt.is_empty());
        
        // Open with correct password
        let recovered_key = open_keystore(password, &keystore).unwrap();
        assert_eq!(master_key, recovered_key);
        
        // Open with wrong password should fail
        let result = open_keystore("wrong_password", &keystore);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_keystore() {
        let password = "test_password";
        let (keystore, _) = create_keystore(password).unwrap();
        
        let new_keys = vec![StoredKey {
            id: "test-key-1".to_string(),
            key_type: "ed25519".to_string(),
            encrypted_key: vec![1, 2, 3, 4],
            public_key: vec![5, 6, 7, 8],
            nonce: vec![9, 10, 11, 12],
            salt: vec![13, 14, 15, 16],
            created: "2024-01-01".to_string(),
            description: Some("Test key".to_string()),
        }];
        
        let updated = update_keystore(password, &keystore, new_keys).unwrap();
        
        // Should be able to open with password
        let keys = get_keys(password, &updated).unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, "test-key-1");
    }
}
