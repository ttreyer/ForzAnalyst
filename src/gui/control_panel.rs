use egui::{CtxRef, Ui};
use egui_backend::egui::{self, Button};
use egui_sdl2_gl as egui_backend;
use tinyfiledialogs::{open_file_dialog, save_file_dialog_with_filter};

pub enum ControlAction {
    Load(String),
    Save(String),
}

pub struct ControlPanel {
    record: bool,
    next_race: bool,
    pub action: Option<ControlAction>,
}

impl ControlPanel {
    pub fn new() -> Self {
        Self {
            record: false,
            next_race: false,
            action: None,
        }
    }

    pub fn is_record(&self) -> bool {
        self.record
    }

    pub fn want_next_race(&self) -> bool {
        self.record
    }

    pub fn start_race(&mut self) {
        self.record = true;
    }

    pub fn show(&mut self, ctx: &CtxRef) {
        self.action = None; // Reset action to None everytimes!

        egui::Window::new("Control Records")
            .auto_sized()
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.render_record_button(ui);

                    self.render_next_race_button(ui);

                    self.render_load_button(ui);
                    self.render_save_button(ui);
                });
            });
    }

    fn render_record_button(&mut self, ui: &mut Ui) {
        let title = match self.record {
            true => "Stop",
            false => "Start",
        };

        if ui.button(title).clicked() {
            self.record = !self.record;
        }
    }

    fn render_next_race_button(&mut self, ui: &mut Ui) {
        let title = match self.next_race {
            true => "Cancel next race",
            false => "Record next race",
        };

        let btn = Button::new(title);

        if ui.add_enabled(false, btn).clicked() {
            self.next_race = !self.next_race;
        }
    }

    fn render_load_button(&mut self, ui: &mut Ui) {
        let btn = Button::new("Load");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = open_file_dialog(
                "Select telemetry file to open",
                ".",
                Some((&["*.ftm"], "ForzAnalyst telemetry")),
            ) {
                self.action = Some(ControlAction::Load(path));
            }
        }
    }

    fn render_save_button(&mut self, ui: &mut Ui) {
        let btn = Button::new("Save");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = save_file_dialog_with_filter(
                "Select path to store telemetry file",
                "telem.ftm",
                &["*.ftm"],
                "ForzAnalyst telemtry",
            ) {
                self.action = Some(ControlAction::Save(path));
            }
        }
    }
}
