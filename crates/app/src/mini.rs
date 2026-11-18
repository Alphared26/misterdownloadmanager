//! This fork's mini pop-up: a small always-on-top gauge that watches one download while
//! something else fills the screen (a game, a film). It shows four things only — the file's
//! name, its percentage, its speed and its time left — and nothing else, so it stays small.
//!
//! Three gauges, chosen in Options → Downloads → Customize "Download progress" dialog:
//! 0 the half-circle speedometer, 1 the ring, 2 the slim bar.
//!
//! What each gauge's arc shows: the ring and the bar fill by the download's *progress*, so they
//! answer "how far along is this file". Only the speedometer's needle shows *speed*, on a fixed
//! 0–100 Mbit/s scale, so it answers "how fast is it coming in right now" and reads like a car's
//! speedometer rather than a bar that jumps about.

use egui::{Color32, Pos2, Rect, Ui, Vec2};

use crate::winui;

/// The speedometer's full scale: 100 Mbit/s. A faster download pins the needle at the right end
/// rather than rescaling, so the figure always means the same thing.
const SPEEDOMETER_FULL_BPS: f64 = 100.0 * 1_000_000.0;

/// How big each gauge's pop-up is, in points: small enough to watch a game or a film around.
/// These track the mock-up (`mockup-popup.html`): the round gauges are near-square, the bar is a
/// wide short strip.
pub fn size(style: u8) -> [f32; 2] {
    match style {
        0 => [240.0, 168.0],
        1 => [196.0, 196.0],
        _ => [240.0, 96.0],
    }
}

/// Everything the gauge draws, read from the download each frame.
pub struct Readout {
    pub file_name: String,
    /// 0 to 1, or None without a size to take a share of.
    pub fraction: Option<f64>,
    /// Bytes a second, as the list and the progress window show it.
    pub rate: f64,
    /// "2m 14s", or empty while it can't be told.
    pub time_left: String,
}

impl Readout {
    /// The percentage, as a 0–100 whole number for the big figure. None without a size: no
    /// percentage rather than 0 %.
    pub fn percent(&self) -> Option<u32> {
        self.fraction.map(|f| (f * 100.0).round() as u32)
    }

    /// How far the ring or the bar is filled, 0 to 1: the download's progress. Without a size
    /// there is nothing to fill.
    pub fn progress(&self) -> f32 {
        self.fraction.unwrap_or(0.0).clamp(0.0, 1.0) as f32
    }

    /// Where the speedometer's needle sits, 0 to 1, on its fixed 0–100 Mbit/s scale.
    pub fn speed_needle(&self) -> f32 {
        if self.rate <= 0.0 {
            return 0.0;
        }
        (self.rate / SPEEDOMETER_FULL_BPS).clamp(0.0, 1.0) as f32
    }
}

/// The accent colour, from the theme so a light or dark palette both read.
fn accent() -> Color32 {
    crate::theme::colors().accent
}

/// Draw the gauge for `style` in `rect`.
pub fn draw(ui: &mut Ui, style: u8, rect: Rect, r: &Readout) {
    match style {
        0 => speedometer(ui, rect, r),
        1 => ring(ui, rect, r),
        _ => bar(ui, rect, r),
    }
}

/// The percentage, big, in the theme's ink.
fn big_percent(ui: &mut Ui, at: Pos2, r: &Readout) {
    let p = ui.painter().clone();
    let text = match r.percent() {
        Some(v) => format!("{v}%"),
        None => crate::i18n::t("—").to_owned(),
    };
    winui::text(&p, at.x, at.y, &text, winui::TEXT());
}

/// The file's name, one line, cut with an ellipsis when too long.
fn file_name(ui: &mut Ui, rect: Rect, r: &Readout, top: f32) {
    let p = ui.painter().clone();
    let max_w = rect.width() - 24.0;
    let mut name = r.file_name.clone();
    while winui::text_width(&p, &name) > max_w && name.chars().count() > 4 {
        name.pop();
    }
    if name != r.file_name {
        while winui::text_width(&p, &format!("{name}…")) > max_w && name.chars().count() > 1 {
            name.pop();
        }
        name.push('…');
    }
    winui::text(&p, rect.left() + 12.0, top, &name, winui::TEXT());
}

/// Speed and time left, on one muted line: "4.200 MB/sec · 2m 14s left".
fn speed_line(ui: &mut Ui, at: Pos2, r: &Readout) {
    let p = ui.painter().clone();
    let speed = crate::model::rate(r.rate);
    let line = if r.time_left.is_empty() {
        speed
    } else {
        crate::i18n::fmt(
            crate::i18n::t("{speed} · {left} left"),
            &[("speed", &speed), ("left", &r.time_left)],
        )
    };
    winui::text(&p, at.x, at.y, &line, winui::MUTED());
}

/// Points along an arc, `begin` clockwise by `sweep`. egui's painter has no arc primitive, so
/// every gauge here is a polyline — many points, so it reads as a smooth curve at this size.
fn arc_points(center: Pos2, radius: f32, begin: f32, sweep: f32, steps: usize) -> Vec<Pos2> {
    (0..=steps)
        .map(|i| {
            let a = begin + sweep * (i as f32 / steps as f32);
            center + Vec2::new(a.cos(), a.sin()) * radius
        })
        .collect()
}

/// The arc's track (the unfilled ring behind the value), drawn with rounded ends like the
/// mock-up's SVG `stroke-linecap="round"`.
fn arc_track(p: &egui::Painter, center: Pos2, radius: f32, width: f32, begin: f32, sweep: f32) {
    p.line(
        arc_points(center, radius, begin, sweep, 96),
        egui::Stroke::new(width, winui::GROUP_LINE()),
    );
}

/// The filled part of the arc, from `begin` clockwise by `sweep * share`, in the accent colour.
/// One flat colour: the mock-up's gradient is a detail that costs segmenting and reads worse at
/// this size, and the flat arc plus the big figure is what carries the value.
fn arc_fill(p: &egui::Painter, center: Pos2, radius: f32, width: f32, begin: f32, sweep: f32, share: f32) {
    if share <= 0.001 {
        return;
    }
    p.line(
        arc_points(center, radius, begin, sweep * share, 96),
        egui::Stroke::new(width, accent()),
    );
}

/// 0: the half-circle speedometer — a needle over a semicircular scale of speed.
fn speedometer(ui: &mut Ui, rect: Rect, r: &Readout) {
    let p = ui.painter().clone();
    let share = r.speed_needle();
    let center = Pos2::new(rect.center().x, rect.top() + 96.0);
    let radius = 76.0;
    // Half a turn, from the left across the top to the right.
    let begin = -std::f32::consts::PI;
    let sweep = std::f32::consts::PI;
    arc_track(&p, center, radius, 13.0, begin, sweep);
    arc_fill(&p, center, radius, 13.0, begin, sweep, share);
    // The needle, from the hub out, turned by the speed across the half.
    let angle = begin + sweep * share;
    let tip = center + Vec2::new(angle.cos(), angle.sin()) * (radius - 18.0);
    p.line_segment([center, tip], egui::Stroke::new(3.0, winui::TEXT()));
    p.circle_filled(center, 6.0, winui::TEXT());
    p.circle_filled(center, 2.5, winui::BG());

    file_name(ui, rect, r, rect.top() + 12.0);
    big_percent(ui, Pos2::new(rect.center().x - 22.0, rect.top() + 100.0), r);
    speed_line(ui, Pos2::new(rect.left() + 12.0, rect.bottom() - 22.0), r);
}

/// 1: the ring — a full circle filled by the download's progress, the percentage inside it.
fn ring(ui: &mut Ui, rect: Rect, r: &Readout) {
    let p = ui.painter().clone();
    use std::f32::consts::TAU;
    let center = rect.center() + Vec2::new(0.0, 4.0);
    let radius = 62.0;
    let begin = -TAU / 4.0;
    arc_track(&p, center, radius, 13.0, begin, TAU);
    arc_fill(&p, center, radius, 13.0, begin, TAU, r.progress());

    // The percentage in the middle, the speed and the time left under it, the file above.
    let pct = match r.percent() {
        Some(v) => format!("{v}%"),
        None => crate::i18n::t("—").to_owned(),
    };
    let w = winui::text_width(&p, &pct);
    winui::text(&p, center.x - w / 2.0, center.y - 24.0, &pct, winui::TEXT());
    let speed = crate::model::rate(r.rate);
    let sw = winui::text_width(&p, &speed);
    winui::text(&p, center.x - sw / 2.0, center.y + 8.0, &speed, winui::MUTED());
    if !r.time_left.is_empty() {
        let left = crate::i18n::fmt(crate::i18n::t("{left} left"), &[("left", &r.time_left)]);
        let lw = winui::text_width(&p, &left);
        winui::text(&p, center.x - lw / 2.0, center.y + 28.0, &left, winui::MUTED());
    }
    file_name(ui, rect, r, rect.top() + 12.0);
}

/// 2: the slim bar — the shortest of the three, for several at once. Filled by progress.
fn bar(ui: &mut Ui, rect: Rect, r: &Readout) {
    let p = ui.painter().clone();
    file_name(ui, rect, r, rect.top() + 12.0);
    big_percent(ui, Pos2::new(rect.left() + 12.0, rect.top() + 32.0), r);
    // The speed, right of the percentage.
    let speed = crate::model::rate(r.rate);
    let sw = winui::text_width(&p, &speed);
    winui::text(
        &p,
        rect.right() - sw - 12.0,
        rect.top() + 46.0,
        &speed,
        winui::MUTED(),
    );
    // The bar.
    let track = Rect::from_min_max(
        Pos2::new(rect.left() + 12.0, rect.bottom() - 24.0),
        Pos2::new(rect.right() - 12.0, rect.bottom() - 12.0),
    );
    p.rect_filled(track, 6.0, winui::GROUP_LINE());
    let share = r.progress();
    if share > 0.0 {
        let fill = Rect::from_min_max(
            track.min,
            Pos2::new(track.min.x + track.width() * share, track.max.y),
        );
        p.rect_filled(fill, 6.0, accent());
    }
    speed_line(ui, Pos2::new(rect.left() + 12.0, rect.bottom() - 10.0), r);
}
