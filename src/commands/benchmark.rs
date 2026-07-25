//! Benchmark command handler

use std::time::{Duration, Instant};

use anyhow::Result;
use colored::Colorize;

use crate::crypto::{
    symmetric::{generate_key, encrypt_aes256gcm, decrypt_aes256gcm, encrypt_chacha20poly1305, decrypt_chacha20poly1305},
    kdf::derive_key_argon2id,
    signatures::{generate_ed25519_keypair, sign, verify},
};

const BENCHMARK_DATA_SIZE: usize = 10 * 1024 * 1024; // 10 MB
const ITERATIONS: usize = 5;

/// Handle benchmark command
pub fn handle_benchmark() -> Result<()> {
    println!();
    println!("{}", "⚡ CRYPGRAPHIC BENCHMARK".cyan().bold());
    println!("{}", "═".repeat(50).cyan());
    println!();
    println!("  Test Size: {} MB", BENCHMARK_DATA_SIZE / (1024 * 1024));
    println!("  Iterations: {}", ITERATIONS);
    println!();

    // Generate test data
    let test_data = vec![0u8; BENCHMARK_DATA_SIZE];
    let key = generate_key();

    // Benchmark AES-256-GCM
    println!("{}", "─".repeat(50).cyan());
    println!("  {}", "AES-256-GCM".yellow().bold());
    println!("{}", "─".repeat(50).cyan());
    
    let (enc_time, dec_time, throughput) = benchmark_encryption(
        &key,
        &test_data,
        encrypt_aes256gcm,
        decrypt_aes256gcm,
    )?;
    
    println!("  {:20} {:.3} ms", "Encryption:".cyan(), enc_time);
    println!("  {:20} {:.3} ms", "Decryption:".cyan(), dec_time);
    println!("  {:20} {} MB/s", "Throughput:".cyan(), throughput);
    println!();

    // Benchmark ChaCha20-Poly1305
    println!("{}", "─".repeat(50).cyan());
    println!("  {}", "ChaCha20-Poly1305".yellow().bold());
    println!("{}", "─".repeat(50).cyan());
    
    let (enc_time, dec_time, throughput) = benchmark_encryption(
        &key,
        &test_data,
        encrypt_chacha20poly1305,
        decrypt_chacha20poly1305,
    )?;
    
    println!("  {:20} {:.3} ms", "Encryption:".cyan(), enc_time);
    println!("  {:20} {:.3} ms", "Decryption:".cyan(), dec_time);
    println!("  {:20} {} MB/s", "Throughput:".cyan(), throughput);
    println!();

    // Benchmark Ed25519 signing
    println!("{}", "─".repeat(50).cyan());
    println!("  {}", "Ed25519 Signing".yellow().bold());
    println!("{}", "─".repeat(50).cyan());
    
    let (sign_time, verify_time) = benchmark_signing(&test_data)?;
    
    println!("  {:20} {:.3} ms", "Signing:".cyan(), sign_time);
    println!("  {:20} {:.3} ms", "Verification:".cyan(), verify_time);
    println!();

    // Benchmark Argon2id
    println!("{}", "─".repeat(50).cyan());
    println!("  {}", "Argon2id KDF".yellow().bold());
    println!("{}", "─".repeat(50).cyan());
    
    let kdf_time = benchmark_kdf(b"test_password")?;
    
    println!("  {:20} {:.3} ms", "Key Derivation:".cyan(), kdf_time);
    println!();

    // Key generation benchmark
    println!("{}", "─".repeat(50).cyan());
    println!("  {}", "Key Generation".yellow().bold());
    println!("{}", "─".repeat(50).cyan());
    
    let keygen_time = benchmark_keygen()?;
    
    println!("  {:20} {:.3} ms", "Ed25519 KeyGen:".cyan(), keygen_time);
    println!();

    println!("{}", "═".repeat(50).cyan());
    println!("  {} Benchmark complete", "✓".green());
    println!("{}", "═".repeat(50).cyan());
    println!();

    Ok(())
}

type EncryptFn = fn(&[u8], &[u8]) -> Result<crate::crypto::EncryptedData, crate::error::CryptoError>;
type DecryptFn = fn(&[u8], &[u8], &[u8]) -> Result<Vec<u8>, crate::error::CryptoError>;

fn benchmark_encryption<F1, F2>(
    key: &[u8],
    data: &[u8],
    encrypt_fn: F1,
    decrypt_fn: F2,
) -> Result<(f64, f64, f64)>
where
    F1: Fn(&[u8], &[u8]) -> Result<crate::crypto::EncryptedData, crate::error::CryptoError>,
    F2: Fn(&[u8], &[u8], &[u8]) -> Result<Vec<u8>, crate::error::CryptoError>,
{
    let mut enc_times = Vec::with_capacity(ITERATIONS);
    let mut dec_times = Vec::with_capacity(ITERATIONS);

    for _ in 0..ITERATIONS {
        // Encryption
        let start = Instant::now();
        let encrypted = encrypt_fn(key, data)?;
        let enc_duration = start.elapsed();
        enc_times.push(enc_duration);

        // Decryption
        let start = Instant::now();
        let decrypted = decrypt_fn(key, &encrypted.nonce, &encrypted.ciphertext)?;
        let dec_duration = start.elapsed();
        dec_times.push(dec_duration);

        // Verify
        assert_eq!(data.len(), decrypted.len());
    }

    let avg_enc = average(&enc_times) * 1000.0; // Convert to ms
    let avg_dec = average(&dec_times) * 1000.0;
    let throughput = (BENCHMARK_DATA_SIZE as f64) / average(&enc_times) / (1024.0 * 1024.0);

    Ok((avg_enc, avg_dec, throughput))
}

fn benchmark_signing(data: &[u8]) -> Result<(f64, f64)> {
    let keypair = generate_ed25519_keypair()?;
    
    let mut sign_times = Vec::with_capacity(ITERATIONS);
    let mut verify_times = Vec::with_capacity(ITERATIONS);
    let mut signature = None;

    for _ in 0..ITERATIONS {
        // Signing
        let start = Instant::now();
        let sig = sign(data, &keypair.secret_key)?;
        let sign_duration = start.elapsed();
        sign_times.push(sign_duration);
        signature = Some(sig);
    }

    let sig = signature.unwrap();
    for _ in 0..ITERATIONS {
        // Verification
        let start = Instant::now();
        let _ = verify(data, &sig, &keypair.public_key)?;
        let verify_duration = start.elapsed();
        verify_times.push(verify_duration);
    }

    let avg_sign = average(&sign_times) * 1000.0;
    let avg_verify = average(&verify_times) * 1000.0;

    Ok((avg_sign, avg_verify))
}

fn benchmark_kdf(password: &[u8]) -> Result<f64> {
    let salt = crate::crypto::kdf::generate_salt();
    let mut times = Vec::with_capacity(3); // Fewer iterations for KDF due to slowness

    for _ in 0..3 {
        let start = Instant::now();
        let _ = derive_key_argon2id(password, &salt)?;
        let duration = start.elapsed();
        times.push(duration);
    }

    Ok(average(&times) * 1000.0)
}

fn benchmark_keygen() -> Result<f64> {
    let mut times = Vec::with_capacity(3);

    for _ in 0..3 {
        let start = Instant::now();
        let _ = generate_ed25519_keypair()?;
        let duration = start.elapsed();
        times.push(duration);
    }

    Ok(average(&times) * 1000.0)
}

fn average(times: &[Duration]) -> f64 {
    let sum: Duration = times.iter().sum();
    sum.as_secs_f64() / times.len() as f64
}
