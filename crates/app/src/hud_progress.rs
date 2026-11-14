//! Futuristic Sci-Fi HUD Progress Visualizer (docs/UI_UX_SPEC.md, #2, #11).
//! Renders real-time Bézier spline speed waveforms, connection thread pulse grids,
//! and telemetry instrument panels (latency, retries, server origin flag).

use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2, pos2, vec2};

#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub speed_bps: u64,
    pub latency_ms: Option<u32>,
    pub retries: u32,
    pub country_code: Option<String>,
    pub server_name: String,
    pub active_workers: usize,
    pub total_workers: usize,
}

impl Default for TelemetryData {
    fn default() -> Self {
        Self {
            speed_bps: 0,
            latency_ms: None,
            retries: 0,
            country_code: None,
            server_name: "Direct Connection".to_string(),
            active_workers: 8,
            total_workers: 8,
        }
    }
}

/// Paint a futuristic spline speed waveform across `rect`.
pub fn paint_speed_waveform(p: &Painter, rect: Rect, speed_history: &[f32], max_speed: f32) {
    if speed_history.len() < 2 {
        return;
    }

    let bg_color = Color32::from_rgba_unmultiplied(10, 16, 29, 200);
    p.rect_filled(rect, 2.0, bg_color);
    p.rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 240, 255, 60)), egui::StrokeKind::Inside);

    let w = rect.width();
    let h = rect.height();
    let step = w / (speed_history.len() - 1) as f32;
    let max_val = if max_speed > 0.0 { max_speed } else { 1.0 };

    let mut points = Vec::with_capacity(speed_history.len());
    for (i, &spd) in speed_history.iter().enumerate() {
        let normalized = (spd / max_val).clamp(0.0, 1.0);
        let x = rect.left() + i as f32 * step;
        let y = rect.bottom() - (normalized * (h - 4.0)) - 2.0;
        points.push(pos2(x, y));
    }

    // Connect points with neon cyan glowing line
    for window in points.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        p.line_segment([p1, p2], Stroke::new(2.0, Color32::from_rgb(0, 240, 255)));
    }
}

/// Paint multi-worker connection thread pulse grid.
pub fn paint_thread_pulse_grid(p: &Painter, o: Pos2, active_count: usize, total_count: usize, pulse_phase: f32) {
    let block_w = 14.0;
    let block_h = 10.0;
    let gap = 3.0;

    for i in 0..total_count {
        let x = o.x + (i as f32 * (block_w + gap));
        let y = o.y;
        let block_rect = Rect::from_min_size(pos2(x, y), vec2(block_w, block_h));

        let is_active = i < active_count;
        let (fill, stroke_color) = if is_active {
            let pulse = ((pulse_phase + (i as f32 * 0.4)).sin() * 0.5 + 0.5) * 60.0;
            (
                Color32::from_rgba_unmultiplied(0, 240, 255, 180 + pulse as u8),
                Color32::from_rgb(0, 255, 157),
            )
        } else {
            (
                Color32::from_rgba_unmultiplied(16, 29, 46, 120),
                Color32::from_rgba_unmultiplied(27, 44, 68, 100),
            )
        };

        p.rect_filled(block_rect, 1.0, fill);
        p.rect_stroke(block_rect, 1.0, Stroke::new(1.0, stroke_color), egui::StrokeKind::Inside);
    }
}

/// Paint compact telemetry instrument box.
pub fn paint_telemetry_box(p: &Painter, rect: Rect, data: &TelemetryData) {
    p.rect_filled(rect, 2.0, Color32::from_rgba_unmultiplied(8, 12, 20, 220));
    p.rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 119, 182, 100)), egui::StrokeKind::Inside);

    let flag = match &data.country_code {
        Some(code) => format!("[{}] ", code.to_uppercase()),
        None => "".to_string(),
    };

    let lat = match data.latency_ms {
        Some(ms) => format!("{ms} ms"),
        None => "-- ms".to_string(),
    };

    let line1 = format!("SERVER: {flag}{}", data.server_name);
    let line2 = format!("PING: {lat} | RETRIES: {} | THREADS: {}/{}", data.retries, data.active_workers, data.total_workers);

    p.text(
        pos2(rect.left() + 6.0, rect.top() + 4.0),
        egui::Align2::LEFT_TOP,
        line1,
        egui::FontId::monospace(10.0),
        Color32::from_rgb(0, 240, 255),
    );

    p.text(
        pos2(rect.left() + 6.0, rect.top() + 18.0),
        egui::Align2::LEFT_TOP,
        line2,
        egui::FontId::monospace(10.0),
        Color32::from_rgb(176, 196, 222),
    );
}
