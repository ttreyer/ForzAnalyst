use eframe::egui;
use egui::CtxRef;

use crate::forza;

#[derive(Default)]
pub struct PacketPanel;

impl PacketPanel {
    pub fn show(&mut self, ctx: &CtxRef, packet: Option<&forza::Packet>) {
        egui::SidePanel::right("Packet").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match packet {
                Some(p) => ui.label(format!("{:#?}", p)),
                None => ui.label("No packet"),
            });
        });
    }
}
