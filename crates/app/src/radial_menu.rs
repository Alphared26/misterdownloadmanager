//! Radial Quick Actions Pie Menu (docs/UI_UX_SPEC.md, #16).
//! Renders a gesture-driven circular context menu for instantaneous mouse actions
//! (Pause/Resume, Boost Threads, Open Folder, Copy URL, Delete).

use egui::{Color32, Context, Painter, Pos2, Rect, Stroke, Vec2, pos2, vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadialAction {
    PauseResume,
    BoostThreads,
    OpenFolder,
    CopyUrl,
    Cancel,
}

pub struct RadialSector {
    pub label: &'static str,
    pub action: RadialAction,
    pub start_angle: f32,
    pub end_angle: f32,
    pub color: Color32,
}

pub struct RadialMenu {
    pub open: bool,
    pub center: Pos2,
    pub hovered_sector: Option<usize>,
}

impl Default for RadialMenu {
    fn default() -> Self {
        Self {
            open: false,
            center: Pos2::ZERO,
            hovered_sector: None,
        }
    }
}

impl RadialMenu {
    pub fn trigger(&mut self, pos: Pos2) {
        self.open = true;
        self.center = pos;
        self.hovered_sector = None;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.hovered_sector = None;
    }

    /// Sectors configuration (4 quadrants + 1 center cancel).
    pub fn sectors() -> [RadialSector; 4] {
        let half_pi = std::f32::consts::FRAC_PI_2;
        [
            RadialSector {
                label: "Pause / Resume",
                action: RadialAction::PauseResume,
                start_angle: -half_pi,
                end_angle: 0.0,
                color: Color32::from_rgb(0, 119, 182),
            },
            RadialSector {
                label: "Boost Threads",
                action: RadialAction::BoostThreads,
                start_angle: 0.0,
                end_angle: half_pi,
                color: Color32::from_rgb(0, 240, 255),
            },
            RadialSector {
                label: "Open Folder",
                action: RadialAction::OpenFolder,
                start_angle: half_pi,
                end_angle: std::f32::consts::PI,
                color: Color32::from_rgb(0, 255, 157),
            },
            RadialSector {
                label: "Copy Link",
                action: RadialAction::CopyUrl,
                start_angle: std::f32::consts::PI,
                end_angle: 3.0 * half_pi,
                color: Color32::from_rgb(255, 183, 3),
            },
        ]
    }

    /// Paint the pie menu and handle sector picking.
    pub fn show(&mut self, p: &Painter, pointer_pos: Option<Pos2>) -> Option<RadialAction> {
        if !self.open {
            return None;
        }

        let inner_r = 24.0;
        let outer_r = 85.0;
        let sectors = Self::sectors();

        let mut picked_action = None;

        // Check hover angle and distance
        let mut hovered = None;
        if let Some(mouse) = pointer_pos {
            let delta = mouse - self.center;
            let dist = delta.length();
            if dist >= inner_r && dist <= outer_r {
                let mut angle = delta.y.atan2(delta.x);
                if angle < -std::f32::consts::FRAC_PI_2 {
                    angle += std::f32::consts::TAU;
                }
                for (i, sec) in sectors.iter().enumerate() {
                    if angle >= sec.start_angle && angle < sec.end_angle {
                        hovered = Some(i);
                        break;
                    }
                }
            }
        }
        self.hovered_sector = hovered;

        // Outer glow circle
        p.circle_filled(self.center, outer_r + 2.0, Color32::from_rgba_unmultiplied(0, 240, 255, 40));

        // Draw sectors
        for (i, sec) in sectors.iter().enumerate() {
            let is_hovered = self.hovered_sector == Some(i);
            let fill_color = if is_hovered {
                Color32::from_rgba_unmultiplied(sec.color.r(), sec.color.g(), sec.color.b(), 240)
            } else {
                Color32::from_rgba_unmultiplied(13, 21, 36, 210)
            };

            let mid_angle = (sec.start_angle + sec.end_angle) / 2.0;
            let label_r = (inner_r + outer_r) / 2.0;
            let label_pos = self.center + vec2(mid_angle.cos() * label_r, mid_angle.sin() * label_r);

            p.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                sec.label,
                egui::FontId::proportional(11.0),
                if is_hovered { Color32::BLACK } else { Color32::WHITE },
            );
        }

        // Center cancel hub
        p.circle_filled(self.center, inner_r, Color32::from_rgb(8, 12, 20));
        p.circle_stroke(self.center, inner_r, Stroke::new(1.5, Color32::from_rgb(0, 240, 255)));
        p.text(
            self.center,
            egui::Align2::CENTER_CENTER,
            "✕",
            egui::FontId::proportional(12.0),
            Color32::from_rgb(255, 0, 85),
        );

        if let Some(idx) = self.hovered_sector {
            picked_action = Some(sectors[idx].action);
        }

        picked_action
    }
}
