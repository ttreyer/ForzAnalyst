use std::collections::HashMap;
use std::mem::replace;

use eframe::egui;
use egui::{CtxRef, Ui};

use crate::{
    dialog,
    event::{Event, EventGenerator},
};

#[repr(u8)]
pub enum ControlPanelEvent {
    Load(String),
    Save(String),
}

pub struct ControlPanel {
    record: bool,
    next_race: bool,
    events: HashMap<u8, Event>,
}

impl ControlPanel {
    pub fn new() -> Self {
        Self {
            record: bool::default(),
            next_race: bool::default(),
            events: HashMap::with_capacity(3),
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
                self.events.insert(
                    ControlPanelEvent::Load as u8,
                    Event::ControlPanelEvent(ControlPanelEvent::Load(path)),
                );
            }
        }
    }

    fn render_save_button(&mut self, ui: &mut Ui) {
        let btn = egui::Button::new("Save");

        if ui.add_enabled(true, btn).clicked() {
            //do something to load a save file
            if let Some(path) = dialog::save_file_dialog() {
                self.events.insert(
                    ControlPanelEvent::Save as u8,
                    Event::ControlPanelEvent(ControlPanelEvent::Save(path)),
                );
            }
        }
    }
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl EventGenerator for ControlPanel {
    fn retrieve_events(&mut self) -> Option<HashMap<u8, Event>> {
        if self.events.is_empty() {
            return None;
        }

        let events = replace(&mut self.events, HashMap::with_capacity(3));
        Some(events)
    }
}
