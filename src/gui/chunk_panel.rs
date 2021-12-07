use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{ForzaChunk, ForzaGameMode};

pub struct ChunkPanel {}

impl ChunkPanel {
    pub fn new() -> Self {
        Self{}
    }
    pub fn show(ctx: &egui::CtxRef, chunks: &LinkedList<ForzaChunk>) {
        egui::Window::new("Chunk").show(ctx, |ui| {
            for chunk in chunks {
                match chunk.game_mode() {
                    ForzaGameMode::FreeRoam => {
                        ui.selectable_label(false, "Free Roam");
                    }
                    ForzaGameMode::Race => {
                        egui::CollapsingHeader::new("Race")
                            .selectable(true)
                            .selected(false)
                            .show(ui, |ui| {
                                for i in 1..=3 {
                                    ui.selectable_label(false, format!("Lap {}", i));
                                }
                            });
                    }
                }
            }
        });
    }
}
