#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod drives;
mod extract;
mod fat32;
mod format;
mod github;

use app::InstallerApp;
use eframe::egui;

// SpruceOS background color
const BG_COLOR: egui::Color32 = egui::Color32::from_rgb(45, 45, 45);

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([400.0, 300.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "SpruceOS Installer",
        options,
        Box::new(|cc| {
            // Set the clear color for the viewport
            cc.egui_ctx.set_visuals(egui::Visuals {
                panel_fill: BG_COLOR,
                window_fill: BG_COLOR,
                extreme_bg_color: BG_COLOR,
                faint_bg_color: BG_COLOR,
                ..egui::Visuals::dark()
            });
            Ok(Box::new(InstallerApp::new(cc)))
        }),
    )
}
