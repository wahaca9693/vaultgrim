//! Operation report generation

use chrono::Local;
use colored::Colorize;

/// Report for encryption/decryption operations
#[derive(Debug, Clone)]
pub struct OperationReport {
    pub operation: String,
    pub input_file: String,
    pub output_file: String,
    pub algorithm: String,
    pub key_type: String,
    pub input_size: u64,
    pub output_size: u64,
    pub duration_ms: u128,
    pub success: bool,
    pub message: Option<String>,
}

impl OperationReport {
    /// Create a new report
    pub fn new(operation: &str, input_file: &str) -> Self {
        Self {
            operation: operation.to_string(),
            input_file: input_file.to_string(),
            output_file: String::new(),
            algorithm: String::new(),
            key_type: String::new(),
            input_size: 0,
            output_size: 0,
            duration_ms: 0,
            success: false,
            message: None,
        }
    }

    /// Set output file
    pub fn with_output_file(mut self, file: &str) -> Self {
        self.output_file = file.to_string();
        self
    }

    /// Set algorithm
    pub fn with_algorithm(mut self, algo: &str) -> Self {
        self.algorithm = algo.to_string();
        self
    }

    /// Set key type
    pub fn with_key_type(mut self, key_type: &str) -> Self {
        self.key_type = key_type.to_string();
        self
    }

    /// Set file sizes
    pub fn with_sizes(mut self, input: u64, output: u64) -> Self {
        self.input_size = input;
        self.output_size = output;
        self
    }

    /// Set duration
    pub fn with_duration(mut self, ms: u128) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Set success
    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Set message
    pub fn with_message(mut self, msg: &str) -> Self {
        self.message = Some(msg.to_string());
        self
    }

    /// Format bytes for display
    fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Format duration for display
    fn format_duration(ms: u128) -> String {
        if ms < 1000 {
            format!("{} ms", ms)
        } else {
            format!("{:.2} s", ms as f64 / 1000.0)
        }
    }

    /// Print the report to console
    pub fn print(&self) {
        println!();
        println!("{}", "═".repeat(60).cyan());
        println!("  {} {}", self.operation.to_uppercase().green().bold(), "REPORT".cyan());
        println!("{}", "═".repeat(60).cyan());
        println!();
        
        // Status
        let status = if self.success {
            format!("{} SUCCESS", "✓").green()
        } else {
            format!("{} FAILED", "✗").red()
        };
        println!("  {:20} {}", "Status:".cyan(), status);
        
        // Timestamp
        println!("  {:20} {}", "Time:".cyan(), Local::now().format("%Y-%m-%d %H:%M:%S"));
        
        // Files
        println!();
        println!("  {:20} {}", "Input:".cyan(), self.input_file);
        if !self.output_file.is_empty() {
            println!("  {:20} {}", "Output:".cyan(), self.output_file);
        }
        
        // Sizes
        if self.input_size > 0 {
            println!();
            println!("  {:20} {}", "Input Size:".cyan(), Self::format_bytes(self.input_size));
            if self.output_size > 0 {
                println!("  {:20} {}", "Output Size:".cyan(), Self::format_bytes(self.output_size));
                let ratio = (self.output_size as f64 / self.input_size as f64 * 100.0) - 100.0;
                let ratio_str = if ratio >= 0.0 {
                    format!("+{:.1}%", ratio).yellow().to_string()
                } else {
                    format!("{:.1}%", ratio).green().to_string()
                };
                println!("  {:20} {} ({} overhead)", "Size Change:".cyan(), ratio_str, Self::format_bytes(self.output_size.saturating_sub(self.input_size)));
            }
        }
        
        // Algorithm info
        if !self.algorithm.is_empty() {
            println!();
            println!("  {:20} {}", "Algorithm:".cyan(), self.algorithm);
        }
        if !self.key_type.is_empty() {
            println!("  {:20} {}", "Key Type:".cyan(), self.key_type);
        }
        
        // Duration
        if self.duration_ms > 0 {
            println!();
            println!("  {:20} {}", "Duration:".cyan(), Self::format_duration(self.duration_ms));
            if self.input_size > 0 {
                let throughput = (self.input_size as f64 / (self.duration_ms as f64 / 1000.0)) as u64;
                println!("  {:20} {}/s", "Throughput:".cyan(), Self::format_bytes(throughput));
            }
        }
        
        // Message
        if let Some(msg) = &self.message {
            println!();
            println!("  {:20} {}", "Message:".cyan(), msg);
        }
        
        println!();
        println!("{}", "═".repeat(60).cyan());
    }
}

/// Print a key generation report
pub fn print_keygen_report(key_type: &str, public_key_file: Option<&str>, private_key_file: Option<&str>) {
    println!();
    println!("{}", "═".repeat(60).green());
    println!("  {} {}", "KEY GENERATION".green().bold(), "COMPLETE".cyan());
    println!("{}", "═".repeat(60).green());
    println!();
    println!("  {:20} {}", "Key Type:".cyan(), key_type);
    if let Some(pk) = public_key_file {
        println!("  {:20} {}", "Public Key:".cyan(), pk);
    }
    if let Some(sk) = private_key_file {
        println!("  {:20} {}", "Private Key:".cyan(), sk);
    }
    println!();
    println!("{}", "⚠️  IMPORTANT: Store private keys securely!".yellow().bold());
    println!();
    println!("{}", "═".repeat(60).green());
}

/// Print a signature report
pub fn print_signature_report(input_file: &str, signature_file: &str, verified: bool) {
    println!();
    if verified {
        println!("{} {}", "✓".green().bold(), "SIGNATURE VALID".green());
    } else {
        println!("{} {}", "✗".red().bold(), "SIGNATURE INVALID".red());
    }
    println!("  File: {}", input_file);
    println!("  Signature: {}", signature_file);
}
