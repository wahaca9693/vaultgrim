//! Secure file deletion
//! 
//! Overwrites file contents before deletion to prevent recovery.

use rand::RngCore;
use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

use crate::error::FileError;

/// Number of overwrite passes (DoD 5220.22-M standard)
const OVERWRITE_PASSES: usize = 3;

/// Securely delete a file by overwriting with random data before removal
pub fn secure_delete(path: &Path) -> Result<(), FileError> {
    if !path.exists() {
        return Ok(());
    }

    let metadata = fs::metadata(path)
        .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to get metadata: {}", e)))?;

    // Don't try to secure delete directories
    if metadata.is_dir() {
        return Err(FileError::SecureDeleteFailed(
            "Cannot secure delete a directory".to_string(),
        ));
    }

    let file_size = metadata.len();

    // Open file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to open file: {}", e)))?;

    let mut rng = rand::rngs::OsRng;
    let block_size = 64 * 1024; // 64KB blocks

    // Overwrite passes
    for pass in 0..OVERWRITE_PASSES {
        file.seek(SeekFrom::Start(0))
            .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to seek: {}", e)))?;

        let mut remaining = file_size as usize;

        // Pattern-based passes as per DoD 5220.22-M
        let pattern = match pass {
            0 => Some(0x00u8),  // Zero
            1 => Some(0xFFu8),  // Ones  
            _ => None,          // Random
        };

        while remaining > 0 {
            let chunk_size = remaining.min(block_size);
            let chunk = if let Some(p) = pattern {
                vec![p; chunk_size]
            } else {
                let mut buf = vec![0u8; chunk_size];
                rng.fill_bytes(&mut buf);
                buf
            };

            file.write_all(&chunk)
                .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to write: {}", e)))?;

            remaining -= chunk_size;
        }

        file.flush()
            .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to flush: {}", e)))?;
    }

    // Sync to disk
    file.sync_all()
        .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to sync: {}", e)))?;

    // Close file
    drop(file);

    // Remove file
    fs::remove_file(path)
        .map_err(|e| FileError::SecureDeleteFailed(format!("Failed to remove file: {}", e)))?;

    Ok(())
}

/// Securely delete a directory and all its contents recursively
pub fn secure_delete_recursive(path: &Path) -> Result<(), FileError> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_file() {
        return secure_delete(path);
    }

    // Walk directory and delete files
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_file() {
            secure_delete(entry_path)?;
        }
    }

    // Remove empty directories (from bottom up)
    let mut dirs: Vec<_> = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    
    dirs.sort_by(|a, b| b.depth().cmp(&a.depth()));
    
    for entry in dirs {
        let dir_path = entry.path();
        if dir_path != path {
            fs::remove_dir(dir_path)
                .map_err(|e| FileError::SecureDeleteFailed(format!(
                    "Failed to remove directory {}: {}", dir_path.display(), e
                )))?;
        }
    }

    // Remove the root directory
    fs::remove_dir(path)
        .map_err(|e| FileError::SecureDeleteFailed(format!(
            "Failed to remove root directory {}: {}", path.display(), e
        )))?;

    Ok(())
}
