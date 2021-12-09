use std::ops::Mul;

use crate::egui_backend::egui;
use crate::forza::forza_packet::ForzaPacket;

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
                x: -1940.0,
                y: 450.0,
            },
            image_size: [3200.0, 1800.0].into(),
            scale: 6.0,
        }
    }

    pub fn show<'a>(&mut self, ctx: &egui::CtxRef, packets: impl Iterator<Item = &'a ForzaPacket>) {
        egui::Window::new("Plot").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.image_pos.x, -3000.0..=0.0));
            ui.add(egui::Slider::new(&mut self.image_pos.y, 0.0..=1000.0));
            ui.add(egui::Slider::new(&mut self.scale, 5.0..=7.0));

            let points = packets.map(|p| Value::new(p.position_x, p.position_z));
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
