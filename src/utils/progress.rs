//! Progress bar and progress tracking

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a new progress bar for file operations
pub fn create_file_progress(total_bytes: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total_bytes);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{{spinner:.cyan}} {{msg}} {{bar:40.cyan/dim}} {{bytes}}/{{total_bytes}} ({{bytes_per_sec}}, {{eta}})"
            ,))
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb
}

/// Create an indeterminate progress bar for unknown-length operations
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Create a multi-progress bar for multiple files
pub fn create_multi_progress(message: &str) -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} {msg} {bar:40} {pos}/{len}")
            .unwrap()
            .progress_chars("━━╸  "),
    );
    pb.set_message(message.to_string());
    pb
}

/// Format bytes to human-readable string
pub fn format_bytes(bytes: u64) -> String {
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

/// Format duration to human-readable string
pub fn format_duration(secs: f64) -> String {
    if secs < 1.0 {
        format!("{:.0}ms", secs * 1000.0)
    } else if secs < 60.0 {
        format!("{:.1}s", secs)
    } else if secs < 3600.0 {
        let mins = secs / 60.0;
        format!("{:.1}m", mins)
    } else {
        let hours = secs / 3600.0;
        format!("{:.1}h", hours)
    }
}

/// Calculate throughput
pub fn calculate_throughput(bytes: u64, elapsed_secs: f64) -> String {
    if elapsed_secs <= 0.0 {
        return "0 B/s".to_string();
    }
    let bytes_per_sec = bytes as f64 / elapsed_secs;
    format_bytes(bytes_per_sec as u64) + "/s"
}
