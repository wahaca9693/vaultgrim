//! Error types for Vaultgrim

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    
    #[error("Invalid password")]
    InvalidPassword,
    
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
    
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    #[error("Verification failed")]
    VerificationFailed,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Failed to read file: {0}")]
    ReadError(String),
    
    #[error("Failed to write file: {0}")]
    WriteError(String),
    
    #[error("Secure delete failed: {0}")]
    SecureDeleteFailed(String),
    
    #[error("Invalid file format: {0}")]
    InvalidFormat(String),
}

#[derive(Error, Debug)]
pub enum KeyStoreError {
    #[error("Key store not initialized")]
    NotInitialized,
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Key already exists: {0}")]
    KeyAlreadyExists(String),
    
    #[error("Invalid key store password")]
    InvalidPassword,
    
    #[error("Corrupted key store: {0}")]
    Corrupted(String),
}
