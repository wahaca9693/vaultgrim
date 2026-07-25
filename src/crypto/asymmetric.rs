//! Asymmetric encryption primitives
//! 
//! Provides X25519 key exchange and optional hybrid Post-Quantum encryption with Kyber.

use crate::error::CryptoError;

/// X25519 public key size
pub const X25519_PUBLIC_KEY_SIZE: usize = 32;
/// X25519 secret key size
pub const X25519_SECRET_KEY_SIZE: usize = 32;

/// Kyber-768 public key size
pub const KYBER768_PUBLIC_KEY_SIZE: usize = 1184;
/// Kyber-768 secret key size
pub const KYBER768_SECRET_KEY_SIZE: usize = 2400;
/// Kyber-768 ciphertext size
pub const KYBER768_CIPHERTEXT_SIZE: usize = 1088;

/// X25519 key pair
#[derive(Debug, Clone)]
pub struct X25519KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

/// Hybrid key pair combining X25519 + Kyber (Post-Quantum)
#[derive(Debug, Clone)]
pub struct HybridKeyPair {
    pub x25519: X25519KeyPair,
    pub kyber_public: Vec<u8>,
    pub kyber_secret: Vec<u8>,
}

/// Encapsulated hybrid key for hybrid encryption
#[derive(Debug, Clone)]
pub struct HybridEncappedKey {
    pub ephemeral_public: Vec<u8>,
    pub kyber_ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

/// Generate X25519 key pair using x25519-dalek
pub fn generate_x25519_keypair() -> Result<X25519KeyPair, CryptoError> {
    use x25519_dalek::{PublicKey, StaticSecret};
    
    let secret = StaticSecret::random();
    let public = PublicKey::from(&secret);
    
    Ok(X25519KeyPair {
        public_key: public.as_bytes().to_vec(),
        secret_key: secret.as_bytes().to_vec(),
    })
}

/// Generate hybrid key pair (X25519 + Kyber-768) for Post-Quantum resistance
#[cfg(feature = "pqc")]
pub fn generate_hybrid_keypair() -> Result<HybridKeyPair, CryptoError> {
    use oqs::kem::{Kem, Algorithm};
    
    let x25519 = generate_x25519_keypair()?;
    
    let kyber = Kem::new(Algorithm::Kyber768).map_err(|e| {
        CryptoError::KeyGenerationFailed(format!("Failed to create Kyber: {}", e))
    })?;
    let (public_key, secret_key) = kyber.keypair().map_err(|e| {
        CryptoError::KeyGenerationFailed(format!("Kyber key generation failed: {}", e))
    })?;
    
    Ok(HybridKeyPair {
        x25519,
        kyber_public: public_key.into_vec(),
        kyber_secret: secret_key.into_vec(),
    })
}

#[cfg(not(feature = "pqc"))]
pub fn generate_hybrid_keypair() -> Result<HybridKeyPair, CryptoError> {
    Err(CryptoError::KeyGenerationFailed(
        "Kyber support not compiled. Build with --features pqc".to_string(),
    ))
}

/// Perform X25519 key exchange
pub fn x25519_key_exchange(
    private_key: &[u8],
    peer_public_key: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    use x25519_dalek::{PublicKey, StaticSecret};
    
    if private_key.len() != X25519_SECRET_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(
            "Invalid X25519 private key size".to_string(),
        ));
    }
    if peer_public_key.len() != X25519_PUBLIC_KEY_SIZE {
        return Err(CryptoError::InvalidKeyFormat(
            "Invalid X25519 public key size".to_string(),
        ));
    }

    let secret_array: [u8; 32] = private_key.try_into().map_err(|_| CryptoError::InvalidKeyFormat("Invalid secret key".to_string()))?;
    let secret = StaticSecret::from(secret_array);
    let public_array: [u8; 32] = peer_public_key.try_into().map_err(|_| CryptoError::InvalidKeyFormat("Invalid public key".to_string()))?;
    let public = PublicKey::from(public_array);

    let shared = secret.diffie_hellman(&public);
    
    Ok(shared.as_bytes().to_vec())
}

/// X25519-only encapsulation
pub fn x25519_encapsulate(peer_public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let ephemeral = generate_x25519_keypair()?;
    let shared = x25519_key_exchange(&ephemeral.secret_key, peer_public_key)?;
    Ok((ephemeral.public_key, shared))
}

/// X25519-only decapsulation
pub fn x25519_decapsulate(private_key: &[u8], ephemeral_public: &[u8]) -> Result<Vec<u8>, CryptoError> {
    x25519_key_exchange(private_key, ephemeral_public)
}

/// Hybrid encapsulation: X25519 + Kyber
#[cfg(feature = "pqc")]
pub fn hybrid_encapsulate(peer_kyber_public: &[u8], peer_x25519_public: &[u8]) -> Result<HybridEncappedKey, CryptoError> {
    use oqs::kem::{Kem, Algorithm};
    use sha2::{Digest, Sha256};
    
    let ephemeral_x25519 = generate_x25519_keypair()?;
    let x25519_shared = x25519_key_exchange(&ephemeral_x25519.secret_key, peer_x25519_public)?;
    
    let kyber = Kem::new(Algorithm::Kyber768).map_err(|e| {
        CryptoError::EncryptionFailed(format!("Failed to create Kyber: {}", e))
    })?;
    
    let peer_pk = kyber.public_key_from_bytes(peer_kyber_public)
        .ok_or_else(|| CryptoError::EncryptionFailed("Invalid Kyber public key length".to_string()))?;
    
    let (kyber_ciphertext, kyber_shared) = kyber.encapsulate(&peer_pk).map_err(|e| {
        CryptoError::EncryptionFailed(format!("Kyber encapsulation failed: {}", e))
    })?;
    
    let mut combined = x25519_shared;
    combined.extend_from_slice(kyber_shared.as_ref());
    
    let mut hasher = Sha256::new();
    hasher.update(&combined);
    hasher.update(b"TerminalEncryptionSuite-Hybrid-v1");
    let shared_secret = hasher.finalize().to_vec();
    
    Ok(HybridEncappedKey {
        ephemeral_public: ephemeral_x25519.public_key,
        kyber_ciphertext: kyber_ciphertext.into_vec(),
        shared_secret,
    })
}

/// Hybrid decapsulation: X25519 + Kyber
#[cfg(feature = "pqc")]
pub fn hybrid_decapsulate(
    x25519_secret: &[u8],
    peer_ephemeral_x25519: &[u8],
    kyber_secret: &[u8],
    kyber_ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    use oqs::kem::{Kem, Algorithm};
    use sha2::{Digest, Sha256};
    
    let x25519_shared = x25519_key_exchange(x25519_secret, peer_ephemeral_x25519)?;
    
    let kyber = Kem::new(Algorithm::Kyber768).map_err(|e| {
        CryptoError::DecryptionFailed(format!("Failed to create Kyber: {}", e))
    })?;
    
    let sk = kyber.secret_key_from_bytes(kyber_secret)
        .ok_or_else(|| CryptoError::DecryptionFailed("Invalid Kyber secret key length".to_string()))?;
    let ct = kyber.ciphertext_from_bytes(kyber_ciphertext)
        .ok_or_else(|| CryptoError::DecryptionFailed("Invalid Kyber ciphertext length".to_string()))?;
    
    let shared_secret = kyber.decapsulate(&sk, &ct).map_err(|e| {
        CryptoError::DecryptionFailed(format!("Kyber decapsulation failed: {}", e))
    })?;
    
    let mut combined = x25519_shared;
    combined.extend_from_slice(shared_secret.as_ref());
    
    let mut hasher = Sha256::new();
    hasher.update(&combined);
    hasher.update(b"TerminalEncryptionSuite-Hybrid-v1");
    let final_shared = hasher.finalize().to_vec();
    
    Ok(final_shared)
}

/// Export public key in format suitable for storage
pub fn export_public_key(key: &[u8], key_type: &str) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    format!("-----BEGIN {} PUBLIC KEY-----\n{}\n-----END {} PUBLIC KEY-----",
            key_type,
            STANDARD.encode(key),
            key_type)
}

/// Import public key from storage format
pub fn import_public_key(pem_data: &str, key_type: &str) -> Result<Vec<u8>, CryptoError> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    
    let expected_header = format!("-----BEGIN {} PUBLIC KEY-----", key_type);
    
    if !pem_data.contains(&expected_header) {
        return Err(CryptoError::InvalidKeyFormat(
            format!("Invalid PEM header for {}", key_type),
        ));
    }
    
    let base64_data = pem_data
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    
    STANDARD.decode(&base64_data).map_err(|e| {
        CryptoError::InvalidKeyFormat(format!("Invalid base64: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x25519_key_exchange() {
        let alice = generate_x25519_keypair().unwrap();
        let bob = generate_x25519_keypair().unwrap();
        
        let alice_shared = x25519_key_exchange(&alice.secret_key, &bob.public_key).unwrap();
        let bob_shared = x25519_key_exchange(&bob.secret_key, &alice.public_key).unwrap();
        
        assert_eq!(alice_shared, bob_shared);
        assert_eq!(alice_shared.len(), 32);
    }
}
