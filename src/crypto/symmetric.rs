//! Symmetric encryption primitives
//! 
//! Provides AES-256-GCM and ChaCha20-Poly1305 encryption.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce as ChaChaNonce,
    XChaCha20Poly1305, XNonce,
};
use rand::RngCore;


use crate::error::CryptoError;

/// 256-bit key for AES-256-GCM
pub const AES_KEY_SIZE: usize = 32;
/// 96-bit nonce for AES-GCM
pub const AES_NONCE_SIZE: usize = 12;

/// 256-bit key for ChaCha20
pub const CHACHA_KEY_SIZE: usize = 32;
/// 96-bit nonce for ChaCha20-Poly1305
pub const CHACHA_NONCE_SIZE: usize = 12;
/// 192-bit nonce for XChaCha20-Poly1305
pub const XCHACHA_NONCE_SIZE: usize = 24;

/// Authenticated encryption with associated data (AEAD) result
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub algorithm: EncryptionAlgorithm,
}

/// Supported symmetric encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

impl EncryptionAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncryptionAlgorithm::Aes256Gcm => "AES-256-GCM",
            EncryptionAlgorithm::ChaCha20Poly1305 => "ChaCha20-Poly1305",
            EncryptionAlgorithm::XChaCha20Poly1305 => "XChaCha20-Poly1305",
        }
    }
}

/// Generate a random 256-bit key
pub fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; AES_KEY_SIZE];
    OsRng.fill_bytes(&mut key);
    key
}

/// Generate a random nonce for the given algorithm
pub fn generate_nonce(algorithm: EncryptionAlgorithm) -> Vec<u8> {
    let mut nonce = vec![0u8; match algorithm {
        EncryptionAlgorithm::Aes256Gcm => AES_NONCE_SIZE,
        EncryptionAlgorithm::ChaCha20Poly1305 => CHACHA_NONCE_SIZE,
        EncryptionAlgorithm::XChaCha20Poly1305 => XCHACHA_NONCE_SIZE,
    }];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Encrypt data using AES-256-GCM
pub fn encrypt_aes256gcm(key: &[u8], plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
    if key.len() != AES_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "AES-256 key must be {} bytes, got {}",
            AES_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let mut nonce_bytes = [0u8; AES_NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
        algorithm: EncryptionAlgorithm::Aes256Gcm,
    })
}

/// Decrypt data using AES-256-GCM
pub fn decrypt_aes256gcm(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != AES_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "AES-256 key must be {} bytes, got {}",
            AES_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed("Authentication failed".to_string()))
}

/// Encrypt data using ChaCha20-Poly1305
pub fn encrypt_chacha20poly1305(key: &[u8], plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
    if key.len() != CHACHA_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "ChaCha20 key must be {} bytes, got {}",
            CHACHA_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let mut nonce_bytes = [0u8; CHACHA_NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = ChaChaNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
        algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
    })
}

/// Decrypt data using ChaCha20-Poly1305
pub fn decrypt_chacha20poly1305(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != CHACHA_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "ChaCha20 key must be {} bytes, got {}",
            CHACHA_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let nonce = ChaChaNonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed("Authentication failed".to_string()))
}

/// Encrypt data using XChaCha20-Poly1305 (extended nonce)
pub fn encrypt_xchacha20poly1305(key: &[u8], plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
    if key.len() != CHACHA_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "XChaCha20 key must be {} bytes, got {}",
            CHACHA_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    let mut nonce_bytes = [0u8; XCHACHA_NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
        algorithm: EncryptionAlgorithm::XChaCha20Poly1305,
    })
}

/// Decrypt data using XChaCha20-Poly1305
pub fn decrypt_xchacha20poly1305(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != CHACHA_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "XChaCha20 key must be {} bytes, got {}",
            CHACHA_KEY_SIZE,
            key.len()
        )));
    }

    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

    let nonce = XNonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed("Authentication failed".to_string()))
}

/// Encrypt with any supported algorithm
pub fn encrypt(algorithm: EncryptionAlgorithm, key: &[u8], plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
    match algorithm {
        EncryptionAlgorithm::Aes256Gcm => encrypt_aes256gcm(key, plaintext),
        EncryptionAlgorithm::ChaCha20Poly1305 => encrypt_chacha20poly1305(key, plaintext),
        EncryptionAlgorithm::XChaCha20Poly1305 => encrypt_xchacha20poly1305(key, plaintext),
    }
}

/// Decrypt with any supported algorithm
pub fn decrypt(algorithm: EncryptionAlgorithm, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    match algorithm {
        EncryptionAlgorithm::Aes256Gcm => decrypt_aes256gcm(key, nonce, ciphertext),
        EncryptionAlgorithm::ChaCha20Poly1305 => decrypt_chacha20poly1305(key, nonce, ciphertext),
        EncryptionAlgorithm::XChaCha20Poly1305 => decrypt_xchacha20poly1305(key, nonce, ciphertext),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes256gcm_encrypt_decrypt() {
        let key = generate_key();
        let plaintext = b"Hello, World! This is a test message.";
        
        let encrypted = encrypt_aes256gcm(&key, plaintext).unwrap();
        let decrypted = decrypt_aes256gcm(&key, &encrypted.nonce, &encrypted.ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_chacha20poly1305_encrypt_decrypt() {
        let key = generate_key();
        let plaintext = b"Hello, World! ChaCha20-Poly1305 test.";
        
        let encrypted = encrypt_chacha20poly1305(&key, plaintext).unwrap();
        let decrypted = decrypt_chacha20poly1305(&key, &encrypted.nonce, &encrypted.ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_xchacha20poly1305_encrypt_decrypt() {
        let key = generate_key();
        let plaintext = b"Hello, World! XChaCha20-Poly1305 test.";
        
        let encrypted = encrypt_xchacha20poly1305(&key, plaintext).unwrap();
        let decrypted = decrypt_xchacha20poly1305(&key, &encrypted.nonce, &encrypted.ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_unique_nonces() {
        let key = generate_key();
        let plaintext = b"test";
        
        let enc1 = encrypt_aes256gcm(&key, plaintext).unwrap();
        let enc2 = encrypt_aes256gcm(&key, plaintext).unwrap();
        
        assert_ne!(enc1.nonce, enc2.nonce);
        assert_ne!(enc1.ciphertext, enc2.ciphertext);
    }
}
