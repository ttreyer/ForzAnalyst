use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{ForzaChunk, ForzaGameMode};

#[derive(PartialEq)]
pub struct ChunkSelection {
    chunk_id: usize,
    lap_id: u16,
}

impl ChunkSelection {
    pub fn new(chunk_id: usize, lap_id: u16) -> Self {
        Self { chunk_id, lap_id }
    }
}

pub struct ChunkPanel {
    pub selection: ChunkSelection,
}

impl ChunkPanel {
    pub fn new() -> Self {
        Self {
            selection: ChunkSelection::new(0, 0),
        }
    }

    fn select(&mut self, chunk_id: usize, lap_id: u16) {
        self.selection = ChunkSelection { chunk_id, lap_id }
    }

    fn is_selected(&self, chunk_id: usize, lap_id: u16) -> bool {
        ChunkSelection { chunk_id, lap_id } == self.selection
    }

    fn show_free_roam(&mut self, ui: &mut egui::Ui, chunk_id: usize) -> egui::Response {
        ui.selectable_label(self.is_selected(chunk_id, 0), "Free Roam")
    }

    fn show_race(
        &mut self,
        ui: &mut egui::Ui,
        chunk_id: usize,
        chunk: &ForzaChunk,
    ) -> egui::Response {
        egui::CollapsingHeader::new("Race")
            .id_source(chunk_id)
            .selectable(true)
            .selected(self.is_selected(chunk_id, 0))
            .default_open(true)
            .show(ui, |ui| {
                for lap_id in 1..=chunk.lap_count() {
                    if ui
                        .selectable_label(
                            self.is_selected(chunk_id, lap_id),
                            format!("Lap {}", lap_id),
                        )
                        .clicked()
                    {
                        self.select(chunk_id, lap_id)
                    }
                }
            })
            .header_response
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, chunks: &LinkedList<ForzaChunk>) {
        egui::Window::new("Chunk").show(ctx, |ui| {
            let mut packets_count = 0usize;

            for (chunk_id, chunk) in chunks.iter().enumerate() {
                packets_count += chunk.packets.len();
                match chunk.game_mode() {
                    ForzaGameMode::FreeRoam => {
                        if self.show_free_roam(ui, chunk_id).clicked() {
                            self.select(chunk_id, 0)
                        }
                    }
                    ForzaGameMode::Race => {
                        if self.show_race(ui, chunk_id, &chunk).clicked() {
                            self.select(chunk_id, 0);
                        }
                    }
                }
            }

            ui.label(format!("Packets: {}", packets_count));
        });
    }
}
