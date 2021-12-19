use std::collections::LinkedList;
use std::mem::replace;
use std::ops::Mul;

use crate::forza;
use eframe::egui;

use egui::plot;
use egui::plot::{PlotImage, Value, Values};
use egui::{TextureId, Vec2};

pub struct MapPanel {
    pointer_coord: Option<Value>,
    image: TextureId,
    image_pos: Value,
    image_size: Vec2,
    scale: f32,
    len: usize,
}

impl Default for MapPanel {
    fn default() -> Self {
        Self {
            pointer_coord: None,
            image: TextureId::default(),
            image_pos: Value {
                x: -1755.0,
                y: 922.0,
            },
            image_size: [5615.0, 3245.0].into(),
            scale: 3.475,
            len: 2000,
        }
    }
}

impl MapPanel {
    pub fn set_image(&mut self, size: Vec2, image: TextureId) {
        self.image_size = size;
        self.image = image;
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, packets: &[forza::Packet]) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add(egui::Slider::new(&mut self.image_pos.x, -2000.0..=-1900.0));
            // ui.add(egui::Slider::new(&mut self.image_pos.y, 400.0..=500.0));
            // ui.add(egui::Slider::new(&mut self.scale, 5.8..=6.2));
            // ui.add(egui::Slider::new(&mut self.len, 0..=10000));

            // let log = packets
            //     .iter()
            //     .filter(|p| !p.position_x.is_normal())
            //     .map(|p| format!("{} {}\n", p.position_x, p.position_z))
            //     .reduce(|a, b| a + &b)
            //     .unwrap_or("<empty>".to_string());
            // egui::Window::new("log").show(ctx, |ui| {
            //     egui::ScrollArea::new([false, true]).show(ui, |ui| {
            //         ui.label(log);
            //     })
            // });

            // The maximum number of points the plot can display is ~10k
            // This step is used to take 1 sample ever `step` to cap the number of points.
            let step = f64::ceil((packets.len() as f64 + 1f64) / self.len as f64) as usize;

            if step > 1 {
                let available_space = ui.available_size_before_wrap();
                let size = egui::vec2(available_space.x, 12.0);
                let (rect, _) = ui.allocate_at_least(size, egui::Sense::hover());

                let painter = ui.painter();

                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(250, 181, 127));
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Data are sampled on the map",
                    egui::TextStyle::Small,
                    egui::Color32::BLACK,
                );
            }

            let mut last_distance = f32::NEG_INFINITY;
            let mut current_line = Vec::new();
            let mut lines = LinkedList::new();
            for p in packets.iter().step_by(step) {
                if p.distance_traveled < replace(&mut last_distance, p.distance_traveled) {
                    lines.push_back(replace(&mut current_line, Vec::new()));
                }

                current_line.push(Value::new(p.position_x, p.position_z));
            }
            lines.push_back(current_line);

            let image_plot =
                PlotImage::new(self.image, self.image_pos, self.image_size.mul(self.scale));

            plot::Plot::new("Map").data_aspect(1.0).show(ui, |plot_ui| {
                self.pointer_coord = match plot_ui.plot_hovered() {
                    true => plot_ui.pointer_coordinate(),
                    false => None,
                };

                let line_color = egui::Color32::from_rgb(255, 0, 255);
                let mut line_width = 4000.0 / plot_ui.plot_bounds().height() as f32;
                if line_width > 100.0 {
                    line_width = 0.1;
                }
                line_width = line_width.clamp(0.5, 12.0);

                plot_ui.image(image_plot);
                for line in lines {
                    plot_ui.line(
                        plot::Line::new(Values::from_values(line))
                            .color(line_color)
                            .width(line_width),
                    );
                }
            });
        });
    }
}
