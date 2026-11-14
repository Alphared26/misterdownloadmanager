//! Mini Floating Always-On-Top Desktop HUD Widget (docs/UI_UX_SPEC.md, #21).
//! Renders an ultra-compact circular or pill HUD over all desktop windows with
//! live download/upload throughput gauges, active download counts, and a direct drag-drop target.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use egui::{Color32, Context, Pos2, Rect, Stroke, Vec2, ViewportBuilder, ViewportCommand, ViewportId, pos2, vec2};

pub const WIDGET_TITLE: &str = "Mister Download Manager HUD Widget";
pub const WIDGET_WIDTH: f32 = 140.0;
pub const WIDGET_HEIGHT: f32 = 140.0;

static WIDGET_VISIBLE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct WidgetTelemetry {
    pub download_speed_bps: u64,
    pub upload_speed_bps: u64,
    pub active_downloads: usize,
    pub total_progress: f32, // 0.0 to 1.0
    pub is_idle: bool,
}

impl Default for WidgetTelemetry {
    fn default() -> Self {
        Self {
            download_speed_bps: 0,
            upload_speed_bps: 0,
            active_downloads: 0,
            total_progress: 0.0,
            is_idle: true,
        }
    }
}

pub struct HudWidget {
    pub pos: Pos2,
    pub visible: bool,
    pub is_dragging: bool,
    pub last_frame: Instant,
    pub pulse_phase: f32,
}

impl Default for HudWidget {
    fn default() -> Self {
        Self {
            pos: pos2(100.0, 100.0),
            visible: false,
            is_dragging: false,
            last_frame: Instant::now(),
            pulse_phase: 0.0,
        }
    }
}

impl HudWidget {
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        WIDGET_VISIBLE.store(self.visible, Ordering::Relaxed);
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Render the independent Always-On-Top viewport HUD.
    pub fn show(&mut self, ctx: &Context, telemetry: &WidgetTelemetry) -> Option<HudWidgetAction> {
        if !self.visible {
            return None;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.pulse_phase = (self.pulse_phase + dt * 3.0) % std::f32::consts::TAU;

        let vid = ViewportId::from_hash_of(WIDGET_TITLE);
        let builder = ViewportBuilder::default()
            .with_title(WIDGET_TITLE)
            .with_always_on_top()
            .with_decorations(false)
            .with_transparent(true)
            .with_inner_size(vec2(WIDGET_WIDTH, WIDGET_HEIGHT))
            .with_resizable(false);

        let mut action = None;

        ctx.show_viewport_immediate(vid, builder, |ui, _| {
            let rect = ui.max_rect();
            let p = ui.painter();
            let center = rect.center();
            let radius = (WIDGET_WIDTH / 2.0) - 8.0;

            // Translucent Obsidian background disk
            let bg_color = Color32::from_rgba_unmultiplied(8, 12, 20, 220);
            p.circle_filled(center, radius, bg_color);

            // Outer Neon Cyan Arc (Download Progress or Pulse)
            let cyan_accent = Color32::from_rgb(0, 240, 255);
            let glow_stroke = Stroke::new(2.5, cyan_accent);
            p.circle_stroke(center, radius, glow_stroke);

            // Secondary Ring (Upload or Activity Pulse)
            let pulse_r = radius - 6.0 + (self.pulse_phase.sin() * 1.5);
            let inner_color = if telemetry.is_idle {
                Color32::from_rgba_unmultiplied(16, 29, 46, 120)
            } else {
                Color32::from_rgba_unmultiplied(0, 255, 157, 180) // Emerald active
            };
            p.circle_stroke(center, pulse_r, Stroke::new(1.5, inner_color));

            // Format speed string
            let speed_str = format_speed(telemetry.download_speed_bps);
            let active_str = format!("{} active", telemetry.active_downloads);

            // Center Telemetry Text
            p.text(
                pos2(center.x, center.y - 10.0),
                egui::Align2::CENTER_CENTER,
                speed_str,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
            p.text(
                pos2(center.x, center.y + 12.0),
                egui::Align2::CENTER_CENTER,
                active_str,
                egui::FontId::proportional(10.0),
                cyan_accent,
            );

            // Dragging & Interaction
            let resp = ui.interact(rect, ui.scope_id().with("hud-interact"), egui::Sense::click_and_drag());
            if resp.drag_started() {
                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
            if resp.double_clicked() {
                action = Some(HudWidgetAction::OpenMainWindow);
            }
            if resp.secondary_clicked() {
                action = Some(HudWidgetAction::ToggleWidget);
            }

            // Drop zone response
            for dropped in &ui.ctx().input(|i| i.raw.dropped_files.clone()) {
                let path = dropped.path().display().to_string();
                if !path.is_empty() {
                    action = Some(HudWidgetAction::DroppedPath(path));
                }
            }
        });

        action
    }
}

pub enum HudWidgetAction {
    OpenMainWindow,
    ToggleWidget,
    DroppedPath(String),
}

fn format_speed(bps: u64) -> String {
    let kbs = bps as f64 / 1024.0;
    if kbs >= 1024.0 {
        format!("{:.1} MB/s", kbs / 1024.0)
    } else {
        format!("{:.0} KB/s", kbs)
    }
}
