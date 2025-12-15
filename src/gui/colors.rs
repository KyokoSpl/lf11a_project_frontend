//! Material 3 Dark Color Palette for egui
//!
//! This module defines the Material Design 3 color scheme used throughout the application.

use egui::Color32;

/// Material 3 Dark Theme Color Palette
///
/// Based on Material Design 3 guidelines with a purple primary color.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Material3Colors {
    // Primary colors
    pub primary: Color32,
    pub on_primary: Color32,
    pub primary_container: Color32,
    pub on_primary_container: Color32,

    // Secondary colors
    pub secondary: Color32,
    pub on_secondary: Color32,
    pub secondary_container: Color32,

    // Tertiary
    pub tertiary: Color32,

    // Surface colors
    pub surface: Color32,
    pub surface_dim: Color32,
    pub surface_bright: Color32,
    pub on_surface: Color32,
    pub surface_variant: Color32,
    pub on_surface_variant: Color32,

    // Outline colors
    pub outline: Color32,
    pub outline_variant: Color32,

    // Semantic colors
    pub error: Color32,
    pub on_error: Color32,
    pub success: Color32,
}

impl Default for Material3Colors {
    fn default() -> Self {
        // Material 3 Dark Theme
        Self {
            // Primary - Light purple tones
            primary: Color32::from_rgb(208, 188, 255),
            on_primary: Color32::from_rgb(56, 30, 114),
            primary_container: Color32::from_rgb(79, 55, 139),
            on_primary_container: Color32::from_rgb(234, 221, 255),

            // Secondary - Gray purple tones
            secondary: Color32::from_rgb(204, 194, 220),
            on_secondary: Color32::from_rgb(51, 45, 65),
            secondary_container: Color32::from_rgb(74, 68, 88),

            // Tertiary
            tertiary: Color32::from_rgb(239, 184, 200),

            // Surface - Dark background
            surface: Color32::from_rgb(20, 18, 24),
            surface_dim: Color32::from_rgb(20, 18, 24),
            surface_bright: Color32::from_rgb(59, 56, 62),
            on_surface: Color32::from_rgb(230, 225, 230),
            surface_variant: Color32::from_rgb(44, 40, 49),
            on_surface_variant: Color32::from_rgb(202, 196, 208),

            // Outline
            outline: Color32::from_rgb(147, 143, 153),
            outline_variant: Color32::from_rgb(68, 64, 75),

            // Semantic
            error: Color32::from_rgb(242, 184, 181),
            on_error: Color32::from_rgb(96, 20, 16),
            success: Color32::from_rgb(129, 199, 132),
        }
    }
}

impl Material3Colors {
    /// Create a light theme color palette
    pub fn light() -> Self {
        Self {
            // Primary - Deep purple tones
            primary: Color32::from_rgb(103, 80, 164),
            on_primary: Color32::from_rgb(255, 255, 255),
            primary_container: Color32::from_rgb(234, 221, 255),
            on_primary_container: Color32::from_rgb(33, 0, 93),

            // Secondary - Gray purple tones
            secondary: Color32::from_rgb(98, 91, 113),
            on_secondary: Color32::from_rgb(255, 255, 255),
            secondary_container: Color32::from_rgb(232, 222, 248),

            // Tertiary
            tertiary: Color32::from_rgb(125, 82, 96),

            // Surface - Light background
            surface: Color32::from_rgb(254, 247, 255),
            surface_dim: Color32::from_rgb(222, 216, 225),
            surface_bright: Color32::from_rgb(254, 247, 255),
            on_surface: Color32::from_rgb(28, 27, 31),
            surface_variant: Color32::from_rgb(231, 224, 236),
            on_surface_variant: Color32::from_rgb(73, 69, 79),

            // Outline
            outline: Color32::from_rgb(121, 116, 126),
            outline_variant: Color32::from_rgb(202, 196, 208),

            // Semantic
            error: Color32::from_rgb(179, 38, 30),
            on_error: Color32::from_rgb(255, 255, 255),
            success: Color32::from_rgb(56, 142, 60),
        }
    }

    /// Create a dark theme color palette (same as default)
    pub fn dark() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material3_colors_default() {
        let colors = Material3Colors::default();

        // Verify primary colors
        assert_eq!(colors.primary, Color32::from_rgb(208, 188, 255));
        assert_eq!(colors.on_primary, Color32::from_rgb(56, 30, 114));

        // Verify surface colors
        assert_eq!(colors.surface, Color32::from_rgb(20, 18, 24));
        assert_eq!(colors.on_surface, Color32::from_rgb(230, 225, 230));

        // Verify error colors
        assert_eq!(colors.error, Color32::from_rgb(242, 184, 181));

        // Verify success color
        assert_eq!(colors.success, Color32::from_rgb(129, 199, 132));
    }

    #[test]
    fn test_material3_colors_clone() {
        let colors = Material3Colors::default();
        let cloned = colors; // Copy trait is implemented

        assert_eq!(colors.primary, cloned.primary);
        assert_eq!(colors.surface, cloned.surface);
        assert_eq!(colors.error, cloned.error);
    }

    #[test]
    fn test_material3_colors_copy() {
        let colors = Material3Colors::default();
        let copied = colors;

        assert_eq!(colors.primary, copied.primary);
        assert_eq!(colors.secondary, copied.secondary);
    }
}
