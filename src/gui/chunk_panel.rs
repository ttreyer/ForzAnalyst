use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{ForzaChunk, ForzaGameMode};

#[derive(PartialEq)]
pub struct ChunkSelection {
    chunk_id: usize,
    lap_id: usize,
}

impl ChunkSelection {
    pub fn new(chunk_id: usize, lap_id: usize) -> Self {
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

    fn select(&mut self, chunk_id: usize, lap_id: usize) {
        self.selection = ChunkSelection { chunk_id, lap_id }
    }

    fn is_selected(&self, chunk_id: usize, lap_id: usize) -> bool {
        ChunkSelection { chunk_id, lap_id } == self.selection
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, chunks: &LinkedList<ForzaChunk>) {
        let mut packets_count = 0usize;
        egui::Window::new("Chunk").show(ctx, |ui| {
            for (chunk_id, chunk) in chunks.iter().enumerate() {
                packets_count += chunk.packets.len();
                match chunk.game_mode() {
                    ForzaGameMode::FreeRoam => {
                        if ui
                            .selectable_label(self.is_selected(chunk_id, 0), "Free Roam")
                            .clicked()
                        {
                            self.select(chunk_id, 0)
                        }
                    }
                    ForzaGameMode::Race => {
                        if egui::CollapsingHeader::new("Race")
                            .id_source(chunk_id)
                            .selectable(true)
                            .selected(self.is_selected(chunk_id, 0))
                            .show(ui, |ui| {
                                for lap_id in 1..=3 {
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
                            .clicked()
                        {
                            self.select(chunk_id, 0);
                        }
                    }
                }
            }
            ui.label(format!("Packets: {}", packets_count));
        });
    }
}
