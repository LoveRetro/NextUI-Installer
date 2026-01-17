#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod debug;
mod drives;
mod eject;
mod extract;
mod fat32;
mod format;
mod github;

use app::InstallerApp;
use config::{load_app_icon, COLOR_BG_DARK, WINDOW_MIN_SIZE, WINDOW_SIZE, WINDOW_TITLE};
use eframe::egui;
use std::sync::Arc;

// Function to check and request privileges on non-Windows platforms
#[cfg(not(windows))]
fn check_and_request_privileges() {
    if unsafe { libc::geteuid() } != 0 {
        // We are not running as root. Attempt to relaunch with elevated privileges.
        println!("Requesting administrator privileges to write to disk...");

        if let Ok(current_exe) = std::env::current_exe() {
            #[cfg(target_os = "linux")]
            let relaunch_command = {
                let mut cmd = std::process::Command::new("pkexec");
                cmd.arg(current_exe);
                cmd
            };

            #[cfg(target_os = "macos")]
            let relaunch_command = {
                // On macOS, use osascript to request administrator privileges
                // This will show a system password prompt.
                let script = format!(
                    "do shell script \"{}\" with administrator privileges",
                    current_exe.to_string_lossy().replace('"', "\\\"") // Escape quotes for shell script
                );
                let mut cmd = std::process::Command::new("osascript");
                cmd.arg("-e").arg(script);
                cmd
            };

            // Fallback for other non-Windows, non-Linux, non-macOS platforms
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            let relaunch_command = {
                eprintln!("Elevated privileges needed but not supported on this non-Windows, non-Linux, non-macOS platform.");
                std::process::exit(1); // Exit with error
            };

            let status = relaunch_command.status();

            if let Err(e) = status {
                eprintln!("Failed to relaunch with elevated privileges: {}. Please run manually with appropriate privilege escalation (e.g., sudo).", e);
            }
        } else {
            eprintln!("Could not determine executable path to relaunch.");
        }

        // Exit the current unprivileged process.
        // The new, privileged process will continue if the user authenticates.
        std::process::exit(0);
    }
}

// Windows platforms don't need this function as privilege elevation is handled by the manifest
#[cfg(windows)]
fn check_and_request_privileges() {
    // No-op for Windows
}


fn main() -> eframe::Result<()> {
    // Call the privilege check at the very beginning of main
    check_and_request_privileges();

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([WINDOW_SIZE.0, WINDOW_SIZE.1])
        .with_min_inner_size([WINDOW_MIN_SIZE.0, WINDOW_MIN_SIZE.1])
        .with_resizable(true);

    // Load custom icon if available
    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|cc| {
            // Set initial visuals (theme is fully applied in InstallerApp::new)
            cc.egui_ctx.set_visuals(egui::Visuals {
                panel_fill: COLOR_BG_DARK,
                window_fill: COLOR_BG_DARK,
                extreme_bg_color: COLOR_BG_DARK,
                faint_bg_color: COLOR_BG_DARK,
                ..egui::Visuals::dark()
            });
            Ok(Box::new(InstallerApp::new(cc)))
        }),
    )
}