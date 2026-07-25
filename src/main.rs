//! Vaultgrim
//! 
//! Industry-standard CLI encryption tool with Post-Quantum resistance.
//! Supports AES-256-GCM, ChaCha20-Poly1305, X25519+Kyber hybrid,
//! Ed25519 signatures, and Argon2id key derivation.

mod cli;
mod commands;
mod crypto;
mod error;
mod utils;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::process;

fn main() {
    // Set up panic hook for better error reporting
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("{}: {}", "PANIC".red().bold(), panic_info);
        process::exit(1);
    }));

    if let Err(e) = run() {
        eprintln!("{}: {}", "ERROR".red().bold(), e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    
    match cli.command {
        cli::Commands::Encrypt {
            input,
            output,
            key,
            password,
            algorithm,
            secure_delete,
            verbose,
        } => {
            commands::encrypt::handle_encrypt(
                input,
                output,
                key,
                password,
                algorithm,
                secure_delete,
                verbose,
            )?;
        }
        cli::Commands::Decrypt {
            input,
            output,
            key,
            password,
            verbose,
        } => {
            commands::decrypt::handle_decrypt(input, output, key, password, verbose)?;
        }
        cli::Commands::EncryptDir {
            input_dir,
            output,
            key,
            password,
            algorithm,
            secure_delete,
            verbose,
        } => {
            commands::encrypt_dir::handle_encrypt_dir(
                input_dir,
                output,
                key,
                password,
                algorithm,
                secure_delete,
                verbose,
            )?;
        }
        cli::Commands::Keygen {
            key_type,
            output,
            password,
            show_public,
        } => {
            commands::keygen::handle_keygen(key_type, output, password, show_public)?;
        }
        cli::Commands::Sign {
            input,
            private_key,
            password,
            output,
        } => {
            commands::sign::handle_sign(input, private_key, password, output)?;
        }
        cli::Commands::Verify {
            input,
            signature,
            public_key,
        } => {
            commands::verify::handle_verify(input, signature, public_key)?;
        }
        cli::Commands::KeyImport {
            input,
            key_type,
            output,
            password,
        } => {
            commands::keygen::handle_import(input, key_type, output, password)?;
        }
        cli::Commands::KeyExport {
            key_id,
            output,
            password,
            format,
        } => {
            commands::keygen::handle_export(key_id, output, password, format)?;
        }
        cli::Commands::KeyList { detailed: _ } => {
            commands::keygen::handle_list()?;
        }
        cli::Commands::KeyPassword {
            key_id,
            old_password,
            new_password,
        } => {
            commands::keygen::handle_change_password(key_id, old_password, new_password)?;
        }
        cli::Commands::Benchmark => {
            commands::benchmark::handle_benchmark()?;
        }
        cli::Commands::Menu => {
            commands::interactive::run();
        }
    }

    Ok(())
}
