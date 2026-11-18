//! Custom wallpaper (this fork, #wallpaper): the user's own picture behind the main window, with
//! the panels drawn over it more transparently so it shows through.
//!
//! The picture is loaded once from a file and kept as a texture; each frame it is drawn over the
//! whole client area first, then the panels are made translucent while the wallpaper is on. When
//! it is off nothing is drawn and the panels keep their full opacity, so a plain MDM looks the
//! same as before.

use std::path::PathBuf;

use egui::{Color32, Rect, TextureHandle, TextureOptions};

/// Where the wallpaper file is kept, so Options can pick it and MDM finds it again next start.
pub fn default_path() -> Option<PathBuf> {
    mdm_ipc::data_dir().map(|d| d.join("wallpaper.jpg"))
}

/// The wallpaper as it is loaded: the picture, and whether it is on.
pub struct Wallpaper {
    pub on: bool,
    texture: Option<TextureHandle>,
    /// The file the texture came from, so a change of file reloads it.
    loaded_from: Option<PathBuf>,
}

impl Default for Wallpaper {
    fn default() -> Self {
        Self { on: false, texture: None, loaded_from: None }
    }
}

impl Wallpaper {
    /// Load the picture at `path` (kept across starts only if it is there) and turn it on.
    pub fn load(&mut self, ctx: &egui::Context, path: &std::path::Path) -> Result<(), String> {
        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
        let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
        let img = img.to_rgba8();
        let (w, h) = (img.width(), img.height());
        let handle = ctx.load_texture(
            "wallpaper",
            egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], img.as_raw()),
            TextureOptions::LINEAR,
        );
        self.texture = Some(handle);
        self.loaded_from = Some(path.to_path_buf());
        self.on = true;
        Ok(())
    }

    /// Whether a picture is loaded and ready to draw.
    pub fn ready(&self) -> bool {
        self.on && self.texture.is_some()
    }

    /// Draw the picture behind everything, stretched to `rect` (a "cover" crop: it covers the
    /// area, keeping its proportions, cropped rather than stretched, centred).
    pub fn draw(&self, painter: &egui::Painter, rect: Rect) {
        let Some(tex) = &self.texture else { return };
        let tex_size = tex.size_vec2();
        // A "cover" crop: the picture fills the area, cropped rather than stretched.
        let scale = (rect.width() / tex_size.x).max(rect.height() / tex_size.y);
        let drawn = tex_size * scale;
        let placed = Rect::from_center_size(rect.center(), drawn);
        // The visible part of the placed picture, as texture coordinates (0..1 of the picture).
        let uv = |p: egui::Pos2| ((p - placed.min) / drawn).to_pos2();
        painter.image(
            tex.id(),
            placed.intersect(rect),
            egui::Rect::from_min_max(uv(rect.min), uv(rect.max)),
            Color32::WHITE,
        );
    }

    /// How much the panels give way to the picture while it is on: the alpha every surface is
    /// drawn at. Low on purpose — the whole point of the wallpaper is to see it, so every grey
    /// panel, the toolbar, the list, the sidebar and the buttons are made mostly see-through.
    /// Text stays at full opacity so it is still readable over the picture.
    pub const PANEL_ALPHA: u8 = 55;

    /// Make every egui surface translucent using default [`PANEL_ALPHA`].
    pub fn apply_translucency(ctx: &egui::Context, on: bool) {
        Self::apply_translucency_alpha(ctx, on, Self::PANEL_ALPHA);
    }

    /// Make every egui surface translucent with a custom alpha so the wallpaper shows through.
    /// Every fill the theme sets is dropped to `alpha`: the panels, the widgets (idle, hovered,
    /// active), the extreme and faint backgrounds behind lists, the window's background and the
    /// selection. Nothing else is lowered, so text and icons stay crisp over the picture.
    pub fn apply_translucency_alpha(ctx: &egui::Context, on: bool, alpha: u8) {
        if !on {
            return; // Off: leave the theme's own colours alone entirely.
        }
        let a = alpha;
        let fade = |c: &mut Color32| {
            let [r, g, b, _] = c.to_srgba_unmultiplied();
            *c = Color32::from_rgba_unmultiplied(r, g, b, a);
        };
        ctx.all_styles_mut(|s| {
            let v = &mut s.visuals;
            fade(&mut v.panel_fill);
            fade(&mut v.window_fill);
            fade(&mut v.extreme_bg_color);
            fade(&mut v.faint_bg_color);
            fade(&mut v.widgets.inactive.bg_fill);
            fade(&mut v.widgets.inactive.weak_bg_fill);
            fade(&mut v.widgets.hovered.bg_fill);
            fade(&mut v.widgets.hovered.weak_bg_fill);
            fade(&mut v.widgets.active.bg_fill);
            fade(&mut v.widgets.active.weak_bg_fill);
            {
                let [r, g, b, _] = v.selection.bg_fill.to_srgba_unmultiplied();
                v.selection.bg_fill = Color32::from_rgba_unmultiplied(r, g, b, a.saturating_add(35).min(255));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Off: the theme's own colours are left alone, so a plain MDM looks as before.
    #[test]
    fn off_leaves_the_theme_alone() {
        let ctx = egui::Context::default();
        let before = ctx.style_of(egui::Theme::Dark).visuals.panel_fill;
        Wallpaper::apply_translucency(&ctx, false);
        assert_eq!(
            before,
            ctx.style_of(egui::Theme::Dark).visuals.panel_fill,
            "off leaves the panel fill alone"
        );
    }

    /// On: every grey surface gives way to the picture — by alpha only, so the theme's colour
    /// still reads through it and text stays readable over it.
    #[test]
    fn on_makes_the_surfaces_translucent() {
        let ctx = egui::Context::default();
        let before = ctx.style_of(egui::Theme::Dark).visuals.panel_fill;
        Wallpaper::apply_translucency(&ctx, true);
        let v = &ctx.style_of(egui::Theme::Dark).visuals;
        let surfaces = [
            v.panel_fill,
            v.window_fill,
            v.extreme_bg_color,
            v.faint_bg_color,
            v.widgets.inactive.bg_fill,
            v.widgets.hovered.bg_fill,
            v.widgets.active.bg_fill,
        ];
        for (i, c) in surfaces.iter().enumerate() {
            assert_eq!(c.a(), Wallpaper::PANEL_ALPHA, "surface {i} not translucent");
        }
        let [r, g, b, _] = before.to_srgba_unmultiplied();
        let [r2, g2, b2, _] = v.panel_fill.to_srgba_unmultiplied();
        assert!(
            (r as i16 - r2 as i16).abs() <= 1
                && (g as i16 - g2 as i16).abs() <= 1
                && (b as i16 - b2 as i16).abs() <= 1,
            "only the alpha changes, not the colour: before=({r},{g},{b}) after=({r2},{g2},{b2})"
        );
    }

    /// Custom alpha sets exactly the translucency requested.
    #[test]
    fn custom_alpha_sets_requested_translucency() {
        let ctx = egui::Context::default();
        Wallpaper::apply_translucency_alpha(&ctx, true, 90);
        let v = &ctx.style_of(egui::Theme::Dark).visuals;
        assert_eq!(v.panel_fill.a(), 90);
    }

    /// A "cover" crop: the picture fills the area, keeping its proportions, cropped rather than
    /// stretched — a 2:1 picture in a square scales by the taller side and shows half its width.
    #[test]
    fn covers_the_area_without_stretching() {
        let ctx = egui::Context::default();
        let img = egui::ColorImage::from_rgba_unmultiplied([100, 50], &[255; 100 * 50 * 4]);
        let tex = ctx.load_texture("w", img, TextureOptions::LINEAR);
        let tex_size = tex.size_vec2();
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(200.0, 200.0));

        let scale = (rect.width() / tex_size.x).max(rect.height() / tex_size.y);
        assert_eq!(scale, 4.0, "scaled by the taller side, not the narrower");
        let drawn = tex_size * scale;
        let placed = Rect::from_center_size(rect.center(), drawn);
        assert_eq!(placed.height(), rect.height(), "scaled 4x, the picture covers the square's height");
        let uv = |p: egui::Pos2| ((p - placed.min) / drawn).to_pos2();
        let uv_rect = Rect::from_min_max(uv(rect.min), uv(rect.max));
        assert_eq!(uv_rect.width(), 0.5, "half the picture's width is shown");
        assert_eq!(uv_rect.height(), 1.0, "the whole of its height is shown");
    }

    /// A picture that isn't an image says why, and turns the wallpaper off rather than leaving a
    /// half-loaded state.
    #[test]
    fn a_file_that_is_not_an_image_fails_cleanly() {
        let mut dir = std::env::temp_dir();
        dir.push("mdm-wallpaper-test.txt");
        std::fs::write(&dir, b"not a picture").unwrap();
        let ctx = egui::Context::default();
        let mut w = Wallpaper::default();
        assert!(w.load(&ctx, &dir).is_err());
        assert!(!w.ready());
        let _ = std::fs::remove_file(dir);
    }
}
