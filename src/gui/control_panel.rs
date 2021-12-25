use eframe::egui;
use egui::{CtxRef, Ui};

pub enum ControlAction {
    Load(String),
    Save(String),
}

#[derive(Default)]
pub struct ControlPanel {
    record: bool,
    next_race: bool,
    pub action: Option<ControlAction>,
}

impl ControlPanel {
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
                    self.render_load_button(ui);
                    self.render_save_button(ui);
                });

                ui.horizontal(|ui| {
                    self.render_record_button(ui);
                    self.render_next_race_button(ui);
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
        ui.add_enabled(
            !self.record,
            egui::Checkbox::new(&mut self.next_race, "Race only"),
        );
    }

    fn render_load_button(&mut self, ui: &mut Ui) {
        let btn = egui::Button::new("Load");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Select telemetry file to open")
                .add_filter("ForzAnalyst telemetry", &["ftm"])
                .pick_file()
                .and_then(|path| path.to_str().map(|s| s.to_owned()))
            {
                self.action = Some(ControlAction::Load(path));
            }
        }
    }

    fn render_save_button(&mut self, ui: &mut Ui) {
        let btn = egui::Button::new("Save");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Select where to store telemetry")
                .add_filter("ForzAnalyst telemetry", &["ftm"])
                .save_file()
                .and_then(|path| path.to_str().map(|s| s.to_owned()))
            {
                self.action = Some(ControlAction::Save(path));
            }
        }
    }
}
