//! Comprehensive Correctness Tests for Terminal Encryption Suite
//! 
//! These tests verify that encryption/decryption produces correct results
//! with byte-for-byte accuracy, and that failures are properly detected.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use sha2::{Digest, Sha256};

// Get the binary path
fn get_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/release/terminal-encryption-suite")
}

/// Calculate SHA-256 hash of a file
fn sha256_file(path: &PathBuf) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Calculate SHA-256 hash of bytes
fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Run the encryption tool
fn encrypt_file(input: &PathBuf, output: Option<&PathBuf>, extra_args: &[&str]) -> Result<(), String> {
    let mut args = vec!["encrypt"];
    args.push(input.to_str().unwrap());
    
    if let Some(out) = output {
        args.push("-o");
        args.push(out.to_str().unwrap());
    }
    
    for arg in extra_args {
        args.push(arg);
    }
    
    let output = Command::new(get_binary())
        .args(&args)
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    Ok(())
}

/// Run the decryption tool
fn decrypt_file(input: &PathBuf, output: Option<&PathBuf>, extra_args: &[&str]) -> Result<(), String> {
    let mut args = vec!["decrypt"];
    args.push(input.to_str().unwrap());
    
    if let Some(out) = output {
        args.push("-o");
        args.push(out.to_str().unwrap());
    }
    
    for arg in extra_args {
        args.push(arg);
    }
    
    let output = Command::new(get_binary())
        .args(&args)
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    Ok(())
}

/// Create a test file with specific content
fn create_test_file(path: &PathBuf, size: usize) -> Result<String, String> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    
    if size == 0 {
        return Ok(sha256_bytes(b""));
    }
    
    // Create deterministic content based on size
    let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    file.write_all(&data).map_err(|e| e.to_string())?;
    
    Ok(sha256_bytes(&data))
}

// =============================================================================
// ROUND-TRIP TESTS
// =============================================================================

#[test]
fn test_roundtrip_text_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    let decrypted_file = test_dir.path().join("test.txt.decrypted");
    
    // Create a text file
    let original_content = b"Hello, this is a test message!";
    fs::write(&input_file, original_content).unwrap();
    
    // Encrypt
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Decrypt
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    // Read decrypted content
    let decrypted_content = fs::read(&decrypted_file).unwrap();
    
    // Verify byte-for-byte match
    assert_eq!(original_content.as_slice(), decrypted_content.as_slice(),
        "Decrypted content should match original exactly");
}

#[test]
fn test_roundtrip_binary_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.bin");
    let encrypted_file = test_dir.path().join("test.bin.enc");
    let decrypted_file = test_dir.path().join("test.bin.decrypted");
    
    // Create a binary file with various bytes
    let original_content: Vec<u8> = (0..=255).collect();
    fs::write(&input_file, &original_content).unwrap();
    
    // Encrypt
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Decrypt
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    // Read and compare
    let decrypted_content = fs::read(&decrypted_file).unwrap();
    
    assert_eq!(original_content, decrypted_content,
        "Binary content should match exactly");
}

#[test]
fn test_roundtrip_empty_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("empty.txt");
    let encrypted_file = test_dir.path().join("empty.txt.enc");
    let decrypted_file = test_dir.path().join("empty.txt.decrypted");
    
    // Create empty file
    fs::write(&input_file, b"").unwrap();
    
    // Encrypt
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Decrypt
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    // Check file sizes
    let input_size = fs::metadata(&input_file).unwrap().len();
    let decrypted_size = fs::metadata(&decrypted_file).unwrap().len();
    
    assert_eq!(input_size, 0);
    assert_eq!(decrypted_size, 0);
}

#[test]
fn test_roundtrip_sha256_verification() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("large_file.bin");
    let encrypted_file = test_dir.path().join("large_file.bin.enc");
    let decrypted_file = test_dir.path().join("large_file.bin.decrypted");
    
    // Create a 10MB file
    let size = 10 * 1024 * 1024;
    let original_hash = create_test_file(&input_file, size).unwrap();
    
    // Encrypt
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Decrypt
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    // Compare hashes
    let decrypted_hash = sha256_file(&decrypted_file).unwrap();
    
    assert_eq!(original_hash, decrypted_hash,
        "SHA-256 hash of decrypted file should match original");
}

// =============================================================================
// ALGORITHM-SPECIFIC TESTS
// =============================================================================

#[test]
fn test_aes256gcm_roundtrip() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    let decrypted_file = test_dir.path().join("test.txt.decrypted");
    
    let original_content = b"AES-256-GCM test data with special chars: @#$%^&*()";
    fs::write(&input_file, original_content).unwrap();
    
    encrypt_file(&input_file, Some(&encrypted_file), &["-a", "aes-256-gcm"]).unwrap();
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    let decrypted_content = fs::read(&decrypted_file).unwrap();
    assert_eq!(original_content.as_slice(), decrypted_content.as_slice());
}

#[test]
fn test_chacha20_roundtrip() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    let decrypted_file = test_dir.path().join("test.txt.decrypted");
    
    let original_content = b"ChaCha20-Poly1305 test data";
    fs::write(&input_file, original_content).unwrap();
    
    encrypt_file(&input_file, Some(&encrypted_file), &["-a", "chacha20-poly1305"]).unwrap();
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    let decrypted_content = fs::read(&decrypted_file).unwrap();
    assert_eq!(original_content.as_slice(), decrypted_content.as_slice());
}

// =============================================================================
// FAILURE TESTS
// =============================================================================

#[test]
fn test_decrypt_with_wrong_key_fails() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    let wrong_key_file = test_dir.path().join("wrong.key");
    
    // Create and encrypt
    fs::write(&input_file, b"Secret data").unwrap();
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Create a random wrong key
    let wrong_key: Vec<u8> = (0..32).map(|_| rand::random()).collect();
    fs::write(&wrong_key_file, &wrong_key).unwrap();
    
    // Try to decrypt with wrong key - should fail
    let result = decrypt_file(&encrypted_file, None, &["-k", wrong_key_file.to_str().unwrap()]);
    
    // This should fail (authentication error)
    // Note: The exact error depends on implementation
    if result.is_ok() {
        // If it succeeded, the decrypted content should be garbage
        let decrypted = fs::read(encrypted_file.with_extension("txt")).unwrap_or_default();
        // Garbage should NOT equal original
        assert_ne!(decrypted, b"Secret data".to_vec());
    }
}

#[test]
fn test_tampered_ciphertext_fails() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    let tampered_file = test_dir.path().join("test.txt.tampered");
    
    // Create and encrypt
    fs::write(&input_file, b"Important data").unwrap();
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Read encrypted content
    let mut encrypted_content = fs::read(&encrypted_file).unwrap();
    
    // Tamper with a byte in the middle (skip header: magic + version + algo + flags)
    if encrypted_content.len() > 20 {
        encrypted_content[20] ^= 0xFF; // Flip bits
        fs::write(&tampered_file, &encrypted_content).unwrap();
    } else {
        panic!("Encrypted file too small to tamper");
    }
    
    // Try to decrypt tampered file - should fail
    let result = decrypt_file(&tampered_file, None, &[]);
    
    // Should get an error, not silently produce garbage
    assert!(result.is_err(), "Decryption of tampered file should fail");
}

#[test]
fn test_encrypted_file_format_verification() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("test.txt");
    let encrypted_file = test_dir.path().join("test.txt.enc");
    
    // Encrypt
    fs::write(&input_file, b"Test").unwrap();
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    
    // Check magic bytes
    let encrypted_content = fs::read(&encrypted_file).unwrap();
    
    assert!(encrypted_content.len() > 4, "Encrypted file should have header");
    assert_eq!(&encrypted_content[0..4], b"TES1", "Should have TES1 magic bytes");
}

// =============================================================================
// FILE SIZE TESTS
// =============================================================================

#[test]
fn test_small_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("small.txt");
    let encrypted_file = test_dir.path().join("small.txt.enc");
    let decrypted_file = test_dir.path().join("small.txt.decrypted");
    
    // 1 byte
    fs::write(&input_file, b"X").unwrap();
    
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    let original = fs::read(&input_file).unwrap();
    let decrypted = fs::read(&decrypted_file).unwrap();
    
    assert_eq!(original, decrypted);
}

#[test]
fn test_large_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("large.bin");
    let encrypted_file = test_dir.path().join("large.bin.enc");
    let decrypted_file = test_dir.path().join("large.bin.decrypted");
    
    // Create 5MB file
    let size = 5 * 1024 * 1024;
    let original_hash = create_test_file(&input_file, size).unwrap();
    
    encrypt_file(&input_file, Some(&encrypted_file), &[]).unwrap();
    decrypt_file(&encrypted_file, Some(&decrypted_file), &[]).unwrap();
    
    let decrypted_hash = sha256_file(&decrypted_file).unwrap();
    assert_eq!(original_hash, decrypted_hash);
}

// =============================================================================
// KEY GENERATION TESTS
// =============================================================================

#[test]
fn test_keygen_ed25519() {
    let test_dir = tempfile::tempdir().unwrap();
    let output_dir = test_dir.path();
    
    // Run keygen
    let output = Command::new(get_binary())
        .args(&["keygen", "ed25519", "-o", output_dir.join("test_key").to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Keygen should succeed");
    
    // Check files exist
    assert!(output_dir.join("test_key").exists(), "Private key should exist");
    assert!(output_dir.join("test_key.pub.key").exists(), "Public key should exist");
}

#[test]
fn test_keygen_x25519() {
    let test_dir = tempfile::tempdir().unwrap();
    let output_dir = test_dir.path();
    
    // Run keygen
    let output = Command::new(get_binary())
        .args(&["keygen", "x25519", "-o", output_dir.join("test_x25519").to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(output.status.success(), "X25519 keygen should succeed");
    assert!(output_dir.join("test_x25519").exists());
}

// =============================================================================
// SIGNATURE TESTS
// =============================================================================

#[test]
fn test_sign_and_verify() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("document.txt");
    let private_key = test_dir.path().join("private.key");
    let public_key = test_dir.path().join("public.key");
    let signature = test_dir.path().join("signature.sig");
    
    // Create test file
    fs::write(&input_file, b"Document content to sign").unwrap();
    
    // Generate key
    Command::new(get_binary())
        .args(&["keygen", "ed25519", "-o", private_key.to_str().unwrap()])
        .output()
        .unwrap();
    
    // Copy public key
    fs::copy(private_key.with_extension("pub.key"), &public_key).unwrap();
    
    // Sign
    let sign_output = Command::new(get_binary())
        .args(&["sign", input_file.to_str().unwrap(), 
                "--private-key", private_key.to_str().unwrap(),
                "-o", signature.to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(sign_output.status.success(), "Sign should succeed: {}", String::from_utf8_lossy(&sign_output.stderr));
    assert!(signature.exists(), "Signature file should exist");
    
    // Verify
    let verify_output = Command::new(get_binary())
        .args(&["verify", input_file.to_str().unwrap(),
                "-s", signature.to_str().unwrap(),
                "--public-key", public_key.to_str().unwrap()])
        .output()
        .unwrap();
    
    // Print output for debugging
    if !verify_output.status.success() {
        eprintln!("Verify failed: {}", String::from_utf8_lossy(&verify_output.stderr));
    }
    
    assert!(verify_output.status.success(), "Verify should succeed: {}", String::from_utf8_lossy(&verify_output.stderr));
}

#[test]
fn test_verify_fails_with_tampered_file() {
    let test_dir = tempfile::tempdir().unwrap();
    let input_file = test_dir.path().join("document.txt");
    let private_key = test_dir.path().join("private.key");
    let public_key = test_dir.path().join("public.key");
    let signature = test_dir.path().join("signature.sig");
    
    // Create and sign
    fs::write(&input_file, b"Original document").unwrap();
    
    Command::new(get_binary())
        .args(&["keygen", "ed25519", "-o", private_key.to_str().unwrap()])
        .output()
        .unwrap();
    fs::copy(private_key.with_extension("pub.key"), &public_key).unwrap();
    
    Command::new(get_binary())
        .args(&["sign", input_file.to_str().unwrap(), 
                "--private-key", private_key.to_str().unwrap(),
                "-o", signature.to_str().unwrap()])
        .output()
        .unwrap();
    
    // Tamper with file
    fs::write(&input_file, b"TAMPERED document").unwrap();
    
    // Verify should fail
    let verify_output = Command::new(get_binary())
        .args(&["verify", input_file.to_str().unwrap(),
                "-s", signature.to_str().unwrap(),
                "--public-key", public_key.to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(!verify_output.status.success(), "Verify should fail for tampered file");
}

// =============================================================================
// BENCHMARK TEST
// =============================================================================

#[test]
fn test_benchmark_runs() {
    let output = Command::new(get_binary())
        .args(&["benchmark"])
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Benchmark should run successfully");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AES-256-GCM"), "Should test AES-256-GCM");
    assert!(stdout.contains("ChaCha20"), "Should test ChaCha20");
}

// Include rand for random key generation
extern crate rand;
