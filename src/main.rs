#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod drives;
mod extract;
mod fat32;
mod format;
mod github;

use app::InstallerApp;
use config::{COLOR_BG_DARK, WINDOW_MIN_SIZE, WINDOW_SIZE, WINDOW_TITLE};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_SIZE.0, WINDOW_SIZE.1])
            .with_min_inner_size([WINDOW_MIN_SIZE.0, WINDOW_MIN_SIZE.1])
            .with_resizable(true),
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
