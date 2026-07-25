//! Digital signatures using Ed25519
//! 
//! Provides Ed25519 signing and verification with multi-identity support.

use ed25519_dalek::{
    Signer, SigningKey, Verifier, VerifyingKey,
    Signature as Ed25519Signature,
};
use rand::rngs::OsRng;


use crate::error::CryptoError;

/// Ed25519 public key size
pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
/// Ed25519 secret key size
pub const ED25519_SECRET_KEY_SIZE: usize = 32;
/// Ed25519 signature size
pub const ED25519_SIGNATURE_SIZE: usize = 64;

/// Ed25519 key pair for signing
#[derive(Debug, Clone)]
pub struct Ed25519KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

/// Ed25519 signature
#[derive(Debug, Clone)]
pub struct Signature {
    pub bytes: Vec<u8>,
}

/// Generate Ed25519 signing key pair
pub fn generate_ed25519_keypair() -> Result<Ed25519KeyPair, CryptoError> {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key: VerifyingKey = (&signing_key).into();
    
    Ok(Ed25519KeyPair {
        public_key: verifying_key.as_bytes().to_vec(),
        secret_key: signing_key.to_bytes().to_vec(),
    })
}

/// Sign data with Ed25519
pub fn sign(message: &[u8], secret_key: &[u8]) -> Result<Signature, CryptoError> {
    if secret_key.len() != ED25519_SECRET_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(
            "Ed25519 secret key must be 32 bytes".to_string(),
        ));
    }
    
    let signing_key = SigningKey::from_bytes(secret_key.try_into().map_err(|_| CryptoError::InvalidKeyFormat("Invalid secret key".to_string()))?);
    
    let signature = signing_key.sign(message);
    
    Ok(Signature {
        bytes: signature.to_bytes().to_vec(),
    })
}

/// Verify Ed25519 signature
pub fn verify(message: &[u8], signature: &Signature, public_key: &[u8]) -> Result<bool, CryptoError> {
    if public_key.len() != ED25519_PUBLIC_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(
            "Ed25519 public key must be 32 bytes".to_string(),
        ));
    }
    if signature.bytes.len() != ED25519_SIGNATURE_SIZE {
        return Err(CryptoError::InvalidKeyFormat(
            "Ed25519 signature must be 64 bytes".to_string(),
        ));
    }
    
    let verifying_key = VerifyingKey::from_bytes(public_key.try_into().map_err(|_| CryptoError::InvalidKeyFormat("Invalid public key".to_string()))?).map_err(|e| CryptoError::SignatureError(e.to_string()))?;
    
    let sig_array: [u8; 64] = signature.bytes.as_slice().try_into().map_err(|_| CryptoError::InvalidKeyFormat("Invalid signature".to_string()))?;
    let sig = Ed25519Signature::from_bytes(&sig_array);
    
    Ok(verifying_key.verify(message, &sig).is_ok())
}

/// Export signing key in PEM format (for storage)
pub fn export_secret_key_pem(secret_key: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    format!(
        "-----BEGIN ED25519 PRIVATE KEY-----\n{}\n-----END ED25519 PRIVATE KEY-----",
        STANDARD.encode(secret_key)
    )
}

/// Export public key in PEM format
pub fn export_public_key_pem(public_key: &[u8]) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    format!(
        "-----BEGIN ED25519 PUBLIC KEY-----\n{}\n-----END ED25519 PUBLIC KEY-----",
        STANDARD.encode(public_key)
    )
}

/// Import signing key from PEM format
pub fn import_secret_key_pem(pem: &str) -> Result<Vec<u8>, CryptoError> {
    let pem = pem.trim();
    let base64_content = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.decode(&base64_content)
        .map_err(|e| CryptoError::InvalidKeyFormat(format!("Failed to decode PEM: {}", e)))
}

/// Import public key from PEM format
pub fn import_public_key_pem(pem: &str) -> Result<Vec<u8>, CryptoError> {
    let pem = pem.trim();
    let base64_content = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.decode(&base64_content)
        .map_err(|e| CryptoError::InvalidKeyFormat(format!("Failed to decode PEM: {}", e)))
}

/// Export signature in binary format
pub fn export_signature(signature: &Signature) -> Vec<u8> {
    signature.bytes.clone()
}

/// Import signature from binary format
pub fn import_signature(bytes: &[u8]) -> Result<Signature, CryptoError> {
    if bytes.len() != ED25519_SIGNATURE_SIZE {
        return Err(CryptoError::InvalidKeyFormat(format!(
            "Signature must be {} bytes, got {}",
            ED25519_SIGNATURE_SIZE,
            bytes.len()
        )));
    }
    Ok(Signature { bytes: bytes.to_vec() })
}

/// Export signature in Base64 format (for embedding in JSON, etc.)
pub fn export_signature_base64(signature: &Signature) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    STANDARD.encode(&signature.bytes)
}

/// Import signature from Base64 format
pub fn import_signature_base64(base64_str: &str) -> Result<Signature, CryptoError> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let bytes = STANDARD.decode(base64_str)
        .map_err(|e| CryptoError::InvalidKeyFormat(format!("Failed to decode Base64: {}", e)))?;
    import_signature(&bytes)
}

/// Batch verification (verifies multiple signatures more efficiently)
pub fn batch_verify<'a>(
    messages: &[&'a [u8]],
    signatures: &[Signature],
    public_keys: &[Vec<u8>],
) -> Result<Vec<bool>, CryptoError> {
    if messages.len() != signatures.len() || signatures.len() != public_keys.len() {
        return Err(CryptoError::SignatureError(
            "Messages, signatures, and public keys must have the same length".to_string(),
        ));
    }
    
    let mut results = Vec::with_capacity(messages.len());
    for ((message, signature), public_key) in messages.iter().zip(signatures.iter()).zip(public_keys.iter()) {
        results.push(verify(message, signature, public_key)?);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_ed25519_keypair().unwrap();
        let message = b"Hello, World! This is a test message.";
        
        let signature = sign(message, &keypair.secret_key).unwrap();
        let is_valid = verify(message, &signature, &keypair.public_key).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_invalid_signature() {
        let keypair = generate_ed25519_keypair().unwrap();
        let message = b"Hello, World!";
        
        let signature = sign(message, &keypair.secret_key).unwrap();
        
        // Verify with wrong message should fail
        let is_valid = verify(b"Wrong message", &signature, &keypair.public_key).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_wrong_public_key() {
        let keypair1 = generate_ed25519_keypair().unwrap();
        let keypair2 = generate_ed25519_keypair().unwrap();
        let message = b"Hello, World!";
        
        let signature = sign(message, &keypair1.secret_key).unwrap();
        
        // Verify with wrong public key should fail
        let is_valid = verify(message, &signature, &keypair2.public_key).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_pem_export_import() {
        let keypair = generate_ed25519_keypair().unwrap();
        
        let pem = export_secret_key_pem(&keypair.secret_key);
        let imported = import_secret_key_pem(&pem).unwrap();
        
        assert_eq!(keypair.secret_key, imported);
    }

    #[test]
    fn test_signature_base64() {
        let keypair = generate_ed25519_keypair().unwrap();
        let message = b"Test message";
        
        let signature = sign(message, &keypair.secret_key).unwrap();
        let base64 = export_signature_base64(&signature);
        let imported = import_signature_base64(&base64).unwrap();
        
        assert_eq!(signature.bytes, imported.bytes);
    }
}
