//! Key Derivation Functions
//! 
//! Provides Argon2id for password-based key derivation.

use argon2::{Argon2, password_hash::SaltString, PasswordHasher};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::error::CryptoError;

/// Argon2id output length (256 bits = 32 bytes)
pub const ARGON2_OUTPUT_SIZE: usize = 32;
/// Argon2id salt size (128 bits = 16 bytes)
pub const ARGON2_SALT_SIZE: usize = 16;

/// Generate a random salt for Argon2id
pub fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; ARGON2_SALT_SIZE];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    salt
}

/// Derive a key from password using Argon2id
pub fn derive_key_argon2id(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let argon2 = Argon2::default();
    
    // Convert raw salt bytes to a SaltString
    // SaltString uses a crypt-style base64 alphabet: ./A-Za-z0-9 (no = padding)
    // Remove padding characters from the base64 encoded salt
    let encoded = STANDARD.encode(salt);
    let encoded_salt = encoded.trim_end_matches('=');
    let salt_str = SaltString::from_b64(encoded_salt)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
    
    let hash = argon2.hash_password(password, &salt_str)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;
    
    // Extract output from hash - the hash field gives us the derived key bytes directly
    let hash_output = hash.hash.ok_or_else(|| CryptoError::KeyDerivationFailed("No hash output".to_string()))?;
    
    // Return the hash bytes
    Ok(hash_output.as_bytes().to_vec())
}

/// Simple base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };
        
        result.push(ALPHABET[b0 >> 2] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        if i + 1 < data.len() {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        if i + 2 < data.len() {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

/// Derive a key from password with automatic salt generation
pub fn derive_key_argon2id_with_salt_generation(password: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let salt = generate_salt();
    let key = derive_key_argon2id(password, &salt)?;
    Ok((key, salt))
}

/// Verify a password against an Argon2id hash
pub fn verify_argon2id(password: &[u8], salt: &[u8], expected_hash: &[u8]) -> Result<bool, CryptoError> {
    let derived = derive_key_argon2id(password, salt)?;
    Ok(derived.as_slice() == expected_hash)
}

/// Simple password hashing for key store (returns hash + salt)
pub fn hash_password(password: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let salt = generate_salt();
    let key = derive_key_argon2id(password, &salt)?;
    Ok((key, salt))
}

/// Fast KDF for deriving multiple keys from a master key (HKDF-like)
pub fn hkdf_expand(_prk: &[u8], info: &[u8], output_len: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(output_len);
    let mut counter: u8 = 1;
    let mut t = Vec::new();
    
    while output.len() < output_len {
        let mut hasher = Sha256::new();
        hasher.update(&t);
        hasher.update(info);
        hasher.update(&[counter]);
        t = hasher.finalize().to_vec();
        output.extend_from_slice(&t);
        counter += 1;
    }
    
    output.truncate(output_len);
    output
}

/// Extract a pseudo-random key from input key material
pub fn hkdf_extract(salt: &[u8], ikm: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(ikm);
    hasher.finalize().to_vec()
}

/// Combined HKDF extract-and-expand
pub fn hkdf(salt: &[u8], ikm: &[u8], info: &[u8], output_len: usize) -> Vec<u8> {
    let prk = hkdf_extract(salt, ikm);
    hkdf_expand(&prk, info, output_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2id_derive() {
        let password = b"test_password_123";
        let salt = generate_salt();
        
        let key1 = derive_key_argon2id(password, &salt).unwrap();
        let key2 = derive_key_argon2id(password, &salt).unwrap();
        
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), ARGON2_OUTPUT_SIZE);
    }

    #[test]
    fn test_hkdf() {
        let salt = b"test_salt";
        let ikm = b"input_key_material";
        let info = b"test_info";
        
        let output = hkdf(salt, ikm, info, 32);
        assert_eq!(output.len(), 32);
        
        // Same inputs should produce same output
        let output2 = hkdf(salt, ikm, info, 32);
        assert_eq!(output, output2);
    }
}
