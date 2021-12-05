use imgui::{Ui, Window, im_str, Condition};

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

    pub fn render(&mut self, ui: &Ui) {
        Window::new(im_str!("Control Records"))
        .size([300.0, 50.0], Condition::Never)
        .collapsible(false)
        .scrollable(false)
        .resizable(false)
        .build(&ui, || {
            self.render_record_button(ui);

            ui.same_line(0f32);

            self.render_next_race_button(ui);

            ui.same_line(0f32);

            self.render_load_button(ui);
        });
    }

    fn render_record_button(&mut self, ui: &Ui) {
        let title = match self.record {
            true => im_str!("Stop"),
            false => im_str!("Start"),
        };

        if ui.button(title, [60f32, 20f32]) {
            self.record = !self.record;
        }
    }

    fn render_next_race_button(&mut self, ui: &Ui) {
        let title = match self.next_race {
            true => im_str!("Cancel next race"),
            false => im_str!("Record next race"),
        };

        if ui.button(title, [120f32, 20f32]) {
            self.next_race = !self.next_race;
        }
    }

    fn render_load_button(&mut self, ui: &Ui) {
        if ui.button(im_str!("Load"), [60f32, 20f32]) {
            //do something to load a save file
        }
    }
}