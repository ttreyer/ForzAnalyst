use egui_sdl2_gl as egui_backend;
use egui_backend::egui;
use egui::{CtxRef, Ui};

pub struct ControlPanel {
    record: bool,
    next_race: bool,
}

impl ControlPanel {
    pub fn new() -> Self {
        Self {
            record: false,
            next_race: false,
        }
    }

    pub fn render(&mut self, ctx: &CtxRef) {
        egui::Window::new("Control Records")
            .auto_sized()
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {

                ui.horizontal(|ui| {
                    self.render_record_button(ui);

                    self.render_next_race_button(ui);

                    self.render_load_button(ui);
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

        if ui.button(title).clicked() {
            self.next_race = !self.next_race;
        }
    }

    fn render_load_button(&mut self, ui: &mut Ui) {
        if ui.button("Load").clicked() {
            //do something to load a save file
        }
    }
}
