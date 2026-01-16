use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
use tokio::sync::mpsc;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Embed the 7zr.exe binary directly into our executable
const SEVEN_ZIP_EXE: &[u8] = include_bytes!("../assets/7zr.exe");

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone)]
pub enum ExtractProgress {
    Started,
    Extracting,
    Completed,
    Error(String),
}

pub async fn extract_7z(
    archive_path: &Path,
    dest_dir: &Path,
    progress_tx: mpsc::UnboundedSender<ExtractProgress>,
) -> Result<(), String> {
    let _ = progress_tx.send(ExtractProgress::Started);

    // Verify archive exists
    if !archive_path.exists() {
        return Err(format!("Archive not found: {:?}", archive_path));
    }

    // Ensure destination directory exists
    if !dest_dir.exists() {
        std::fs::create_dir_all(dest_dir)
            .map_err(|e| format!("Failed to create destination directory: {}", e))?;
    }

    let _ = progress_tx.send(ExtractProgress::Extracting);

    // Extract 7zr.exe to temp directory
    let temp_dir = std::env::temp_dir();
    let seven_zip_path = temp_dir.join("7zr_spruce.exe");

    // Write the embedded 7z executable to temp
    std::fs::write(&seven_zip_path, SEVEN_ZIP_EXE)
        .map_err(|e| format!("Failed to extract 7z tool: {}", e))?;

    // Run 7zr.exe to extract the archive
    // Command: 7zr.exe x archive.7z -oDestination -y
    let output_arg = format!("-o{}", dest_dir.display());

    #[cfg(windows)]
    let result = Command::new(&seven_zip_path)
        .arg("x")                           // Extract with full paths
        .arg(archive_path)                  // Archive to extract
        .arg(&output_arg)                   // Output directory
        .arg("-y")                          // Yes to all prompts
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .await;

    #[cfg(not(windows))]
    let result = Command::new(&seven_zip_path)
        .arg("x")
        .arg(archive_path)
        .arg(&output_arg)
        .arg("-y")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    // Clean up the temp 7z executable
    let _ = std::fs::remove_file(&seven_zip_path);

    match result {
        Ok(output) => {
            if output.status.success() {
                let _ = progress_tx.send(ExtractProgress::Completed);
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                let err_msg = format!(
                    "7z extraction failed:\n{}\n{}",
                    stdout.trim(),
                    stderr.trim()
                );
                let _ = progress_tx.send(ExtractProgress::Error(err_msg.clone()));
                Err(err_msg)
            }
        }
        Err(e) => {
            let err_msg = format!("Failed to run 7z: {}", e);
            let _ = progress_tx.send(ExtractProgress::Error(err_msg.clone()));
            Err(err_msg)
        }
    }
}

/// Alias for backward compatibility with app.rs
pub async fn extract_7z_with_progress(
    archive_path: &Path,
    dest_dir: &Path,
    progress_tx: mpsc::UnboundedSender<ExtractProgress>,
) -> Result<(), String> {
    extract_7z(archive_path, dest_dir, progress_tx).await
}
