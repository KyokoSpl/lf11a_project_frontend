//! Reusable UI components for Material 3 design
//!
//! This module provides helper functions for creating consistent UI elements.

use egui::{Button, Frame, Margin, Response, RichText, Rounding, Stroke, TextEdit, Ui, Vec2};
use super::Material3Colors;

/// Creates a Material 3 styled button
/// 
/// # Arguments
/// * `ui` - The egui UI context
/// * `colors` - The Material 3 color palette
/// * `text` - Button label text
/// * `primary` - If true, uses primary color; otherwise uses surface variant
/// 
/// # Returns
/// The button's response for handling clicks
pub fn material_button(ui: &mut Ui, colors: &Material3Colors, text: &str, primary: bool) -> Response {
    let (bg_color, text_color) = if primary { 
        (colors.primary, colors.on_primary) 
    } else { 
        (colors.surface_variant, colors.on_surface) 
    };
    
    let button = Button::new(
        RichText::new(text)
            .color(text_color)
            .size(13.0)
    )
    .fill(bg_color)
    .stroke(Stroke::NONE)
    .rounding(Rounding::same(8.0))
    .min_size(Vec2::new(90.0, 36.0));
    
    ui.add(button)
}

/// Creates a Material 3 styled text input field
/// 
/// # Arguments
/// * `ui` - The egui UI context
/// * `colors` - The Material 3 color palette
/// * `text` - Mutable reference to the string to edit
/// * `hint` - Placeholder hint text
/// 
/// # Returns
/// The text edit's response
pub fn styled_text_input(ui: &mut Ui, colors: &Material3Colors, text: &mut String, hint: &str) -> Response {
    Frame::none()
        .fill(colors.surface)
        .stroke(Stroke::new(1.0, colors.outline_variant))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(12.0, 10.0))
        .show(ui, |ui| {
            ui.add(
                TextEdit::singleline(text)
                    .desired_width(380.0)
                    .hint_text(RichText::new(hint).color(colors.on_surface_variant))
                    .text_color(colors.on_surface)
                    .frame(false)
            )
        }).inner
}

/// Apply Material 3 styling to a ComboBox dropdown scope
/// 
/// # Arguments
/// * `ui` - The egui UI context
/// * `colors` - The Material 3 color palette
/// * `content` - Closure that renders the ComboBox
pub fn styled_dropdown<R>(ui: &mut Ui, colors: &Material3Colors, content: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        // Style the combobox button
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.inactive.bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, colors.outline_variant);
        ui.style_mut().visuals.widgets.inactive.rounding = Rounding::same(8.0);
        ui.style_mut().visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, colors.on_surface);
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.hovered.bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, colors.primary);
        ui.style_mut().visuals.widgets.hovered.rounding = Rounding::same(8.0);
        ui.style_mut().visuals.widgets.active.weak_bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.active.bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::new(2.0, colors.primary);
        ui.style_mut().visuals.widgets.active.rounding = Rounding::same(8.0);
        ui.style_mut().visuals.widgets.open.weak_bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.open.bg_fill = colors.surface;
        ui.style_mut().visuals.widgets.open.bg_stroke = Stroke::new(2.0, colors.primary);
        ui.style_mut().visuals.widgets.open.rounding = Rounding::same(8.0);
        
        // Style the popup menu background
        ui.style_mut().visuals.window_fill = colors.surface;
        ui.style_mut().visuals.window_stroke = Stroke::new(1.0, colors.outline_variant);
        ui.style_mut().visuals.window_rounding = Rounding::same(8.0);
        ui.style_mut().visuals.popup_shadow = egui::epaint::Shadow::NONE;
        
        // Selection highlight color
        ui.style_mut().visuals.selection.bg_fill = colors.primary_container;
        ui.style_mut().visuals.selection.stroke = Stroke::NONE;
        
        ui.style_mut().spacing.combo_height = 40.0;
        // Increase the popup menu height to show more items
        ui.style_mut().spacing.combo_width = 430.0;
        
        content(ui)
    }).inner
}

/// Creates a Material 3 styled card container
/// 
/// # Arguments
/// * `ui` - The egui UI context
/// * `colors` - The Material 3 color palette
/// * `content` - Closure that renders the card's content
#[allow(dead_code)]
pub fn material_card(ui: &mut Ui, colors: &Material3Colors, content: impl FnOnce(&mut Ui)) {
    Frame::none()
        .fill(colors.surface_variant)
        .stroke(Stroke::new(1.0, colors.outline_variant))
        .rounding(Rounding::same(16.0))
        .inner_margin(Margin::same(20.0))
        .outer_margin(Margin::symmetric(0.0, 4.0))
        .show(ui, content);
}

#[cfg(test)]
mod tests {
    use egui::{Color32, Vec2};

    #[test]
    fn test_color32_from_rgb() {
        let color = Color32::from_rgb(255, 128, 64);
        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 128);
        assert_eq!(color.b(), 64);
    }

    #[test]
    fn test_color32_equality() {
        let c1 = Color32::from_rgb(100, 100, 100);
        let c2 = Color32::from_rgb(100, 100, 100);
        let c3 = Color32::from_rgb(200, 200, 200);
        
        assert_eq!(c1, c2);
        assert_ne!(c1, c3);
    }

    #[test]
    fn test_vec2_creation() {
        let v = Vec2::new(100.0, 50.0);
        assert_eq!(v.x, 100.0);
        assert_eq!(v.y, 50.0);
    }
}
