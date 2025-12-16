//! Personnel Management System - Frontend Application
//!
//! A cross-platform desktop application built with egui/eframe for managing
//! employees, departments, and salary grades.

use eframe::egui;
use egui::{IconData, Rounding, Shadow};
use std::sync::{Arc, LockResult};

mod api;
mod config;
mod gui;

#[cfg(test)]
mod tests;

use gui::PersonnelApp;

/// Load the application icon from the embedded SVG.
/// Returns None if the icon cannot be loaded.
fn load_icon() -> Option<IconData> {
    // Include the SVG file at compile time
    let svg_data = include_bytes!("../assets/icon.svg");
    // Parse SVG using resvg
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(svg_data, &options).ok()?;

    // Render to a pixmap
    let size = tree.size();
    let scale = 64.0 / size.width().max(size.height());
    let width = (size.width() * scale) as u32;
    let height = (size.height() * scale) as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)?;

    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Convert RGBA to the format expected by IconData
    let rgba = pixmap.take();

    Some(IconData {
        rgba,
        width,
        height,
    })
}

fn main() -> eframe::Result<()> {
    let _ = config::Config::get();

    // Load the application icon
    let icon = load_icon();

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([800.0, 600.0]);

    // Set the icon if it was loaded successfully
    if let Some(icon_data) = icon {
        viewport_builder = viewport_builder.with_icon(Arc::new(icon_data));
    }

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    eframe::run_native(
        "Personnel Management System",
        options,
        Box::new(|cc| {
            // Set dark theme
            let mut visuals = egui::Visuals::dark();
            visuals.window_rounding = Rounding::same(12.0);
            visuals.window_shadow = Shadow::NONE;
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(PersonnelApp::new()))
        }),
    )
}
