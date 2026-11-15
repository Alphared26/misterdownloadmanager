//! First-time launch welcome modal dialog.
//! Displays modification notice and GitHub repository for Mayolus Gavin Alphared (@Alphared26)
//! while honoring the original author's copyright (Ahmed Nefzaoui) under GNU General Public License v3.0.

use crate::dialogs::{Outcome, window};
use crate::winui::{self, ButtonKind, r};
use egui::{Color32, Context};

pub struct WelcomeDialog;

impl WelcomeDialog {
    pub fn show(ctx: &Context) -> Outcome<()> {
        window(
            ctx,
            "alphared-welcome",
            "Mister Download Manager (Next-Gen)",
            [500.0, 270.0],
            false,
            winui::BG(),
            |ui, o| {
                let p = ui.painter().clone();
                let ink = winui::TEXT();

                // Neon title
                winui::text(
                    &p,
                    o.x + 24.0,
                    o.y + 24.0,
                    "Mister Download Manager :: Next-Gen Edition",
                    Color32::from_rgb(0, 240, 255),
                );

                // Modification notice
                winui::paragraph(
                    &p,
                    o.x + 24.0,
                    o.y + 50.0,
                    "Modified with Next-Gen UI/UX & performance features by Mayolus Gavin Alphared (@Alphared26).\nIncludes Direct Video Popups, Social Media Scraper, Sci-Fi HUD, and Wallpaper engine.",
                    ink,
                    o.x + 476.0,
                    14.0,
                );

                // GitHub Repo Link
                if winui::link(
                    ui,
                    &p,
                    o.x + 24.0,
                    o.y + 115.0,
                    "GitHub: https://github.com/Alphared26/misterdownloadmanager",
                    "alphared_gh_link",
                ) {
                    crate::launch::open("https://github.com/Alphared26/misterdownloadmanager");
                }

                // Original author copyright and attribution
                winui::paragraph(
                    &p,
                    o.x + 24.0,
                    o.y + 148.0,
                    "Original software, architecture & core clean-room design\nCopyright (C) 2026 Ahmed Nefzaoui & contributors (GNU General Public License v3.0).",
                    winui::MUTED(),
                    o.x + 476.0,
                    12.0,
                );

                // Continue / OK button
                let ok = winui::button(
                    ui,
                    r(o, 376.0, 225.0, 100.0, 26.0),
                    "Get Started",
                    ButtonKind::Default,
                    false,
                    None,
                )
                .clicked();

                if ok || winui::escape_pressed(ui) || winui::enter_pressed(ui) {
                    Outcome::Closed
                } else {
                    Outcome::Open
                }
            },
        )
    }
}
