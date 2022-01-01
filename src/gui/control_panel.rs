use eframe::egui;
use egui::{CtxRef, Ui};

use crate::{
    dialog,
    event::{self, EventGenerator},
};

pub enum EventTypes {
    Load(String),
    Save(String),
}
type Events = event::Events<EventTypes>;

#[derive(Default)]
pub struct ControlPanel {
    record: bool,
    next_race: bool,
    events: Events,
}

impl event::EventGenerator<EventTypes> for ControlPanel {
    fn events(&mut self) -> &mut Events {
        &mut self.events
    }
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
            if let Some(path) = dialog::pick_file_dialog() {
                self.gen_event(EventTypes::Load(path));
            }
        }
    }

    fn render_save_button(&mut self, ui: &mut Ui) {
        let btn = egui::Button::new("Save");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = dialog::save_file_dialog() {
                self.gen_event(EventTypes::Save(path));
            }
        }
    }
}
