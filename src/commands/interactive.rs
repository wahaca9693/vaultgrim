//! Interactive menu mode - user selects options by number

use std::io::{self, Write};

pub fn run() {
    loop {
        print_menu();
        
        let choice = get_user_choice();
        
        match choice.as_str() {
            "1" => encrypt_flow(),
            "2" => decrypt_flow(),
            "3" => encrypt_dir_flow(),
            "4" => keygen_flow(),
            "5" => benchmark_flow(),
            "6" => sign_flow(),
            "7" => verify_flow(),
            "8" => key_list_flow(),
            "0" => {
                println!("\n Goodbye!\n");
                break;
            },
            _ => {
                println!("\n Invalid choice. Please enter a number from 0 to 8.\n");
            }
        }
        
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn print_menu() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              Vaultgrim - Interactive Menu               ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║   Select an option by number:                               ║");
    println!("║                                                              ║");
    println!("║   ENCRYPTION                                                ║");
    println!("║      [1] Encrypt a file                                    ║");
    println!("║      [2] Decrypt a file                                    ║");
    println!("║      [3] Encrypt a directory                               ║");
    println!("║                                                              ║");
    println!("║   KEY MANAGEMENT                                            ║");
    println!("║      [4] Generate new keys                                 ║");
    println!("║      [8] List stored keys                                  ║");
    println!("║                                                              ║");
    println!("║   SIGNATURES                                               ║");
    println!("║      [6] Sign a file                                       ║");
    println!("║      [7] Verify a signature                               ║");
    println!("║                                                              ║");
    println!("║   UTILITIES                                                ║");
    println!("║      [5] Benchmark (test speed)                            ║");
    println!("║                                                              ║");
    println!("║   [0] Exit                                                 ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_file_path(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn encrypt_flow() {
    println!("\n ENCRYPT FILE");
    println!("================");
    
    let input_file = get_file_path("Enter file path to encrypt");
    let output_file = get_file_path("Enter output file (or press Enter for auto)");
    
    println!("\nSelect algorithm:");
    println!("  [1] AES-256-GCM (recommended)");
    println!("  [2] ChaCha20-Poly1305 (mobile/IoT)");
    
    let algo_choice = get_user_choice();
    let algorithm = match algo_choice.as_str() {
        "2" => "chacha20-poly1305",
        _ => "aes-256-gcm",
    };
    
    println!("\nRunning encryption...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("encrypt").arg(&input_file).arg("--algorithm").arg(algorithm);
    
    if !output_file.is_empty() {
        cmd.arg("--output").arg(&output_file);
    }
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running encrypt: {}", e),
    }
}

fn decrypt_flow() {
    println!("\n DECRYPT FILE");
    println!("================");
    
    let input_file = get_file_path("Enter encrypted file path");
    let output_file = get_file_path("Enter output file (or press Enter for auto)");
    
    println!("\nRunning decryption...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("decrypt").arg(&input_file);
    
    if !output_file.is_empty() {
        cmd.arg("--output").arg(&output_file);
    }
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running decrypt: {}", e),
    }
}

fn encrypt_dir_flow() {
    println!("\n ENCRYPT DIRECTORY");
    println!("=====================");
    
    let input_dir = get_file_path("Enter directory path");
    let output_file = get_file_path("Enter output archive file (.tar.enc)");
    
    println!("\nRunning directory encryption...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("encrypt-dir").arg(&input_dir).arg("--output").arg(&output_file);
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running encrypt-dir: {}", e),
    }
}

fn keygen_flow() {
    println!("\n KEY GENERATION");
    println!("==================");
    
    println!("Select key type:");
    println!("  [1] X25519 (recommended)");
    println!("  [2] X25519 + Kyber-768 (Post-Quantum)");
    println!("  [3] Ed25519 (signing only)");
    
    let key_choice = get_user_choice();
    let key_type = match key_choice.as_str() {
        "2" => "x25519-kyber",
        "3" => "ed25519",
        _ => "x25519",
    };
    
    let output_file = get_file_path("Enter key file name (or press Enter for default)");
    
    println!("\nGenerating keys...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("keygen").arg("--key-type").arg(key_type);
    
    if !output_file.is_empty() {
        cmd.arg("--output").arg(&output_file);
    }
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running keygen: {}", e),
    }
}

fn benchmark_flow() {
    println!("\n RUNNING BENCHMARK");
    println!("=====================");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("benchmark");
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running benchmark: {}", e),
    }
}

fn sign_flow() {
    println!("\n SIGN FILE");
    println!("==============");
    
    let input_file = get_file_path("Enter file to sign");
    let key_file = get_file_path("Enter private key file");
    
    println!("\nSigning file...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("sign").arg(&input_file).arg("--private-key").arg(&key_file);
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running sign: {}", e),
    }
}

fn verify_flow() {
    println!("\n VERIFY SIGNATURE");
    println!("====================");
    
    let input_file = get_file_path("Enter signed file");
    let signature_file = get_file_path("Enter signature file");
    let public_key = get_file_path("Enter public key file");
    
    println!("\nVerifying signature...");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("verify")
       .arg(&input_file)
       .arg("--signature").arg(&signature_file)
       .arg("--public-key").arg(&public_key);
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running verify: {}", e),
    }
}

fn key_list_flow() {
    println!("\n STORED KEYS");
    println!("===============");
    
    let mut cmd = std::process::Command::new("vaultgrim");
    cmd.arg("key-list");
    
    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => println!("Error running key-list: {}", e),
    }
}
