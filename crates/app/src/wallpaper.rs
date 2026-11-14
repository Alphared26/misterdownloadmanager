//! Custom wallpaper and adaptive contrast engine (#45, docs/UI_UX_SPEC.md).
//! Loads user images (PNG/JPG/WebP), calculates spatial luminance via WCAG / OKLab formulas,
//! and adapts UI elements (frosted glass scrims, high-contrast text, glowing borders).

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

use egui::{Color32, ColorImage, Context, Rect, TextureHandle, TextureOptions, Ui, vec2};
use serde::{Deserialize, Serialize};

static WALLPAPER_ENABLED: AtomicBool = AtomicBool::new(false);
static WALLPAPER_STATE: RwLock<Option<WallpaperState>> = RwLock::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperConfig {
    pub enabled: bool,
    pub path: PathBuf,
    pub opacity: f32,
    pub blur_radius: f32,
    pub brightness: f32,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            path: PathBuf::new(),
            opacity: 0.85,
            blur_radius: 0.0,
            brightness: 1.0,
        }
    }
}

pub struct WallpaperState {
    pub config: WallpaperConfig,
    pub texture: Option<TextureHandle>,
    pub avg_luminance: f32,
    pub is_light: bool,
}

impl WallpaperState {
    pub fn new(config: WallpaperConfig) -> Self {
        Self {
            config,
            texture: None,
            avg_luminance: 0.1, // default dark canvas
            is_light: false,
        }
    }

    /// Calculate perceptual relative luminance (WCAG 2.1 formula).
    pub fn calculate_luminance(pixels: &[u8]) -> (f32, bool) {
        if pixels.is_empty() {
            return (0.1, false);
        }

        let mut sum_lum = 0.0f64;
        let count = pixels.len() / 4;
        let step = (count / 2048).max(1); // Sample up to 2048 pixels for 60fps responsiveness

        let mut sampled = 0;
        for i in (0..pixels.len()).step_by(step * 4) {
            if i + 3 < pixels.len() {
                let r = pixels[i] as f64 / 255.0;
                let g = pixels[i + 1] as f64 / 255.0;
                let b = pixels[i + 2] as f64 / 255.0;

                let r_lin = if r <= 0.04045 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
                let g_lin = if g <= 0.04045 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
                let b_lin = if b <= 0.04045 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };

                let lum = 0.2126 * r_lin + 0.7152 * g_lin + 0.0722 * b_lin;
                sum_lum += lum;
                sampled += 1;
            }
        }

        let avg = if sampled > 0 { (sum_lum / sampled as f64) as f32 } else { 0.1 };
        let is_light = avg > 0.45;
        (avg, is_light)
    }

    /// Load or reload texture into egui context.
    pub fn load_image(&mut self, ctx: &Context, raw_bytes: &[u8], width: usize, height: usize) {
        let (lum, is_light) = Self::calculate_luminance(raw_bytes);
        self.avg_luminance = lum;
        self.is_light = is_light;

        let color_image = ColorImage::from_rgba_unmultiplied([width, height], raw_bytes);
        self.texture = Some(ctx.load_texture("custom_wallpaper", color_image, TextureOptions::LINEAR));
    }
}

/// Paint the wallpaper scaled across the given UI rect, applying frosted glass tint.
pub fn paint_wallpaper(ui: &mut Ui, rect: Rect) {
    if !WALLPAPER_ENABLED.load(Ordering::Relaxed) {
        return;
    }

    let state = WALLPAPER_STATE.read().unwrap();
    if let Some(state) = state.as_ref() {
        if let Some(texture) = &state.texture {
            let p = ui.painter();
            // Paint wallpaper image
            p.image(
                texture.id(),
                rect,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::from_white_alpha((state.config.opacity * 255.0).clamp(0.0, 255.0) as u8),
            );

            // Overlay frosted glass scrim for optimal legibility
            let scrim_color = if state.is_light {
                Color32::from_rgba_unmultiplied(255, 255, 255, 140)
            } else {
                Color32::from_rgba_unmultiplied(8, 12, 20, 180)
            };
            p.rect_filled(rect, 0.0, scrim_color);
        }
    }
}

/// Adaptive colors based on current wallpaper luminance.
pub fn adaptive_text_color() -> Color32 {
    let state = WALLPAPER_STATE.read().unwrap();
    if let Some(state) = state.as_ref() {
        if state.config.enabled {
            return if state.is_light {
                Color32::from_rgb(11, 19, 43) // Dark navy/charcoal for bright backgrounds
            } else {
                Color32::from_rgb(0, 240, 255) // Cyberpunk neon cyan for dark backgrounds
            };
        }
    }
    crate::theme::colors().text
}

/// Adaptive border stroke color based on wallpaper.
pub fn adaptive_border_color() -> Color32 {
    let state = WALLPAPER_STATE.read().unwrap();
    if let Some(state) = state.as_ref() {
        if state.config.enabled {
            return if state.is_light {
                Color32::from_rgba_unmultiplied(0, 119, 182, 180)
            } else {
                Color32::from_rgba_unmultiplied(0, 240, 255, 200)
            };
        }
    }
    crate::theme::colors().field_border
}
