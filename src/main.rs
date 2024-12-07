#![windows_subsystem = "windows"]
mod app;
mod clicker;
mod mouse_button;
mod mouse_mover;

use app::MourseApp;
use eframe::egui::{ViewportBuilder, IconData};
use std::sync::Arc;

fn load_icon() -> Arc<IconData> {
    let image_bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(image_bytes).unwrap().resize(16, 16, image::imageops::FilterType::Nearest).into_rgba8();
    Arc::new(IconData {
        rgba: image.to_vec(),
        width: 16,
        height: 16,
    })
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "Mourse",
        options,
        Box::new(|_cc| Ok(Box::new(MourseApp::default()))),
    )
}