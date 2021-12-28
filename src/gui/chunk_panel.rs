// use std::mem::replace;

use crate::forza::{self, Lap};
use eframe::egui;

pub type ChunkID = usize;
pub type LapID = Option<u16>;

#[derive(PartialEq, Default, Clone, Copy)]
pub struct ChunkSelection(pub ChunkID, pub LapID);

#[derive(Default)]
pub struct ChunkPanel {
    pub selection: ChunkSelection,
    pub trash_chunk: Option<ChunkSelection>,
}

impl ChunkPanel {
    fn select(&mut self, chunk_id: ChunkID, lap_id: LapID) {
        self.selection = ChunkSelection(chunk_id, lap_id)
    }

    fn is_selected(&self, chunk_id: ChunkID, lap_id: LapID) -> bool {
        ChunkSelection(chunk_id, lap_id) == self.selection
    }

    pub fn selected_packets<'a>(&self, chunks: &'a forza::Chunks) -> &'a [forza::Packet] {
        let ChunkSelection(chunk_id, lap_id) = self.selection;
        match (chunks.iter().nth(chunk_id), lap_id) {
            (Some(selected_chunk), Some(lap)) => selected_chunk.lap_packets(lap),
            (Some(selected_chunk), None) => &selected_chunk.packets,
            (None, _) => &[],
        }
    }

    fn trash_chunk(&mut self, chunk_id: ChunkID, lap_id: LapID) {
        self.trash_chunk = Some(ChunkSelection(chunk_id, lap_id))
    }

    fn show_free_roam(&mut self, ui: &mut egui::Ui, chunk_id: ChunkID) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(self.is_selected(chunk_id, None), "Free Roam")
                .clicked()
            {
                self.select(chunk_id, None)
            }

            if ui.button("🗑").clicked() {
                self.trash_chunk(chunk_id, None);
            }
        });
    }

    fn show_race(&mut self, ui: &mut egui::Ui, chunk_id: usize, chunk: &forza::Chunk) {
        let resp = egui::CollapsingHeader::new("Race")
            .id_source(chunk_id)
            .selectable(true)
            .selected(self.is_selected(chunk_id, None))
            .default_open(true)
            .show(ui, |ui| {
                // let mut last_lap = 0u16;
                for Lap(lap_num, _, _) in &chunk.lap_index {
                    // if *lap_num < replace(&mut last_lap, *lap_num) {
                    //     continue;
                    // }
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                self.is_selected(chunk_id, Some(*lap_num)),
                                format!("Lap {}", lap_num + 1),
                            )
                            .clicked()
                        {
                            self.select(chunk_id, Some(*lap_num))
                        }

                        if ui.button("🗑").clicked() {
                            self.trash_chunk(chunk_id, Some(*lap_num));
                        }
                    });
                }
            })
            .header_response;

        if resp.clicked() {
            self.select(chunk_id, None);
        }
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, chunks: &forza::Chunks) {
        self.trash_chunk = None;

        egui::Window::new("Chunk").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut packets_count = 0usize;

                for (chunk_id, chunk) in chunks.iter().enumerate() {
                    packets_count += chunk.packets.len();
                    match chunk.game_mode() {
                        forza::GameMode::FreeRoam => self.show_free_roam(ui, chunk_id),
                        forza::GameMode::Race => self.show_race(ui, chunk_id, &chunk),
                    }
                }

                ui.label(format!("Packets: {}", packets_count));
            });
        });
    }
}
