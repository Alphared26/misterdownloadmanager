//! Modern Synthesizer Sound Design (docs/UI_UX_SPEC.md, #17).
//! Generates subtle, modern notification audio tones (sine-sweep complete chime,
//! soft error chord, queue trigger pulse) procedurally without bulky external WAV assets.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CueKind {
    DownloadComplete,
    DownloadFailed,
    QueueStarted,
}

pub struct SynthTone {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

impl SynthTone {
    /// Generate audio buffer for given cue kind with smooth ADSR envelope.
    pub fn generate(kind: CueKind, sample_rate: u32) -> Self {
        let (duration_sec, start_freq, end_freq) = match kind {
            CueKind::DownloadComplete => (0.35, 523.25, 1046.5), // C5 to C6 pleasant ascending chime
            CueKind::DownloadFailed => (0.40, 311.13, 207.65),   // Eb4 to Ab3 soft descending warning
            CueKind::QueueStarted => (0.15, 659.25, 880.0),      // E5 to A5 brief confirmation blip
        };

        let total_samples = (duration_sec * sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(total_samples);

        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;
            let frac = t / duration_sec;

            // Frequency interpolation
            let freq = start_freq + (end_freq - start_freq) * frac;
            let phase = std::f32::consts::TAU * freq * t;

            // Attack - Decay - Sustain - Release (ADSR) envelope
            let env = if frac < 0.1 {
                frac / 0.1 // Attack
            } else if frac > 0.7 {
                (1.0 - frac) / 0.3 // Release
            } else {
                1.0 // Sustain
            };

            let wave = phase.sin() * env * 0.25; // Gentle output gain
            samples.push(wave);
        }

        Self {
            sample_rate,
            samples,
        }
    }
}
