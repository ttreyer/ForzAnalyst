use std::ops::Mul;

use crate::egui_backend::egui;
use crate::forza;

use egui::plot;
use egui::plot::{PlotImage, Value, Values};
use egui::{TextureId, Vec2};

pub struct MapPanel {
    image: TextureId,
    image_pos: Value,
    image_size: Vec2,
    scale: f32,
}

impl MapPanel {
    pub fn new(image_tex_id: TextureId) -> Self {
        Self {
            image: image_tex_id,
            image_pos: Value {
                x: -1978.0,
                y: 435.0,
            },
            image_size: [3200.0, 1800.0].into(),
            scale: 5.92,
        }
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, packets: &[forza::Packet]) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.image_pos.x, -2000.0..=-1900.0));
            ui.add(egui::Slider::new(&mut self.image_pos.y, 400.0..=500.0));
            ui.add(egui::Slider::new(&mut self.scale, 5.8..=6.2));

            let points = packets
                .iter()
                .map(|p| Value::new(p.position_x, p.position_z));
            let player_track = plot::Line::new(Values::from_values_iter(points));

            let image_plot =
                PlotImage::new(self.image, self.image_pos, self.image_size.mul(self.scale));

            ui.add(
                plot::Plot::new("Map")
                    .data_aspect(1.0)
                    .image(image_plot)
                    .line(player_track),
            );
        });
    }
}
