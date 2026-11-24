#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use kakuru::KakuruApp;

fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/kakuru_icon.png"))
        .expect("KAKURU icon must be a valid PNG");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 760.0])
            .with_min_inner_size([360.0, 560.0])
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "KAKURU",
        options,
        Box::new(|cc| Ok(Box::new(KakuruApp::new(cc)))),
    )
}
