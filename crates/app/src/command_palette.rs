//! Command Palette (Ctrl+K / Spotlight Search) & Smart Filter Chips (docs/UI_UX_SPEC.md).
//! Provides instant keyboard-first fuzzy filtering for active/past downloads,
//! rapid actions (Pause All, Limit Speed, Switch Profiles), and smart tag filtering.

use egui::{Color32, Context, Key, Pos2, Rect, Stroke, Ui, Vec2, pos2, vec2};

#[derive(Debug, Clone)]
pub struct CommandItem {
    pub title: String,
    pub subtitle: String,
    pub category: String,
    pub shortcut: Option<String>,
    pub action: PaletteAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteAction {
    FilterTag(String),
    PauseAll,
    ResumeAll,
    SetSpeedLimit(u64),
    SwitchBandwidthProfile(String),
    SelectDownload(u64),
}

pub struct CommandPalette {
    pub open: bool,
    pub query: String,
    pub selected_index: usize,
    pub active_tag: Option<String>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected_index: 0,
            active_tag: None,
        }
    }
}

impl CommandPalette {
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.query.clear();
            self.selected_index = 0;
        }
    }

    /// Handle Ctrl+K shortcut globally.
    pub fn check_shortcut(&mut self, ctx: &Context) {
        if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::K)) {
            self.toggle();
        }
    }

    /// Render the modal overlay in the main window.
    pub fn ui(&mut self, ui: &mut Ui, available_downloads: &[(u64, String, String)]) -> Option<PaletteAction> {
        if !self.open {
            return None;
        }

        let mut chosen_action = None;
        let screen_rect = ui.ctx().screen_rect();
        let p = ui.painter();

        // Dark modal overlay backdrop
        p.rect_filled(screen_rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 160));

        // Center Palette Window (600x380 px)
        let pal_size = vec2(600.0, 380.0);
        let pal_rect = Rect::from_center_size(screen_rect.center(), pal_size);

        // Frame
        p.rect_filled(pal_rect, 4.0, Color32::from_rgb(13, 21, 36));
        p.rect_stroke(pal_rect, 4.0, Stroke::new(1.0, Color32::from_rgb(0, 240, 255)));

        // Items list
        let mut items = vec![
            CommandItem {
                title: "Resume All Downloads".to_string(),
                subtitle: "Restart all paused downloads in the queue".to_string(),
                category: "Actions".to_string(),
                shortcut: Some("Alt+R".to_string()),
                action: PaletteAction::ResumeAll,
            },
            CommandItem {
                title: "Pause All Downloads".to_string(),
                subtitle: "Temporarily halt active network transfers".to_string(),
                category: "Actions".to_string(),
                shortcut: Some("Alt+P".to_string()),
                action: PaletteAction::PauseAll,
            },
            CommandItem {
                title: "Gaming Mode Bandwidth".to_string(),
                subtitle: "Limit transfer rate to reduce ping spikes".to_string(),
                category: "Profiles".to_string(),
                shortcut: None,
                action: PaletteAction::SwitchBandwidthProfile("Gaming".to_string()),
            },
            CommandItem {
                title: "Night Boost Bandwidth".to_string(),
                subtitle: "Maximum multi-connection speed without throttle".to_string(),
                category: "Profiles".to_string(),
                shortcut: None,
                action: PaletteAction::SwitchBandwidthProfile("NightBoost".to_string()),
            },
        ];

        // Append filtered downloads
        for &(id, ref name, ref category) in available_downloads {
            items.push(CommandItem {
                title: name.clone(),
                subtitle: format!("Category: {category}"),
                category: "Downloads".to_string(),
                shortcut: None,
                action: PaletteAction::SelectDownload(id),
            });
        }

        // Filter items by query
        let query_lower = self.query.to_lowercase();
        let filtered: Vec<&CommandItem> = items
            .iter()
            .filter(|it| {
                if query_lower.is_empty() {
                    true
                } else {
                    it.title.to_lowercase().contains(&query_lower)
                        || it.subtitle.to_lowercase().contains(&query_lower)
                        || it.category.to_lowercase().contains(&query_lower)
                }
            })
            .collect();

        // Keyboard navigation
        if ui.ctx().input(|i| i.key_pressed(Key::ArrowDown)) {
            if self.selected_index + 1 < filtered.len() {
                self.selected_index += 1;
            }
        }
        if ui.ctx().input(|i| i.key_pressed(Key::ArrowUp)) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        }
        if ui.ctx().input(|i| i.key_pressed(Key::Escape)) {
            self.open = false;
        }
        if ui.ctx().input(|i| i.key_pressed(Key::Enter)) && !filtered.is_empty() {
            let idx = self.selected_index.min(filtered.len().saturating_sub(1));
            chosen_action = Some(filtered[idx].action.clone());
            self.open = false;
        }

        chosen_action
    }
}
