// use std::mem::replace;

use std::collections::HashMap;
use std::mem::replace;

use crate::event::{Event, EventGenerator};
use crate::forza::{self, Lap};
use crate::forza::{ChunkId, ChunkSelector, LapId};
use eframe::egui;

pub enum ChunkPanelEvent {
    ChangeSelection(ChunkSelector),
    RemoveChunk(ChunkSelector),
}

pub struct ChunkPanel {
    selection: ChunkSelector,
    events: HashMap<u8, Event>,
}

impl ChunkPanel {
    pub fn new() -> Self {
        Self {
            events: HashMap::with_capacity(1),
            ..Default::default()
        }
    }

    fn select(&mut self, chunk_id: ChunkId, lap_id: LapId) {
        self.selection = ChunkSelector(chunk_id, lap_id);

        self.events.insert(
            ChunkPanelEvent::ChangeSelection as u8,
            Event::ChunkPanelEvent(ChunkPanelEvent::ChangeSelection(self.selection)),
        );
    }

    fn is_selected(&self, chunk_id: ChunkId, lap_id: LapId) -> bool {
        ChunkSelector(chunk_id, lap_id) == self.selection
    }

    pub fn set_selection(&mut self, chunk_selector: ChunkSelector) {
        self.selection = chunk_selector;
    }

    pub fn selected_packets<'a>(&self, chunks: &'a forza::Chunks) -> &'a [forza::Packet] {
        let ChunkSelector(chunk_id, lap_id) = self.selection;
        match (chunks.list().iter().nth(chunk_id), lap_id) {
            (Some(selected_chunk), Some(lap)) => selected_chunk.lap_packets(lap),
            (Some(selected_chunk), None) => &selected_chunk.packets,
            (None, _) => &[],
        }
    }

    fn remove_chunk(&mut self, chunk_id: ChunkId, lap_id: LapId) {
        self.events.insert(
            ChunkPanelEvent::RemoveChunk as u8,
            Event::ChunkPanelEvent(ChunkPanelEvent::RemoveChunk(ChunkSelector(
                chunk_id, lap_id,
            ))),
        );
    }

    fn show_free_roam(&mut self, ui: &mut egui::Ui, chunk_id: ChunkId) {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(self.is_selected(chunk_id, None), "Free Roam")
                .clicked()
            {
                self.select(chunk_id, None)
            }

            if ui.button("🗑").clicked() {
                self.remove_chunk(chunk_id, None);
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
                            self.select(chunk_id, Some(*lap_num));
                        }

                        if ui.button("🗑").clicked() {
                            self.remove_chunk(chunk_id, Some(*lap_num));
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
        egui::Window::new("Chunk").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut packets_count = 0usize;

                for (chunk_id, chunk) in chunks.list().iter().enumerate() {
                    packets_count += chunk.packets.len();
                    match chunk.game_mode() {
                        forza::GameMode::FreeRoam => self.show_free_roam(ui, chunk_id),
                        forza::GameMode::Race => self.show_race(ui, chunk_id, &chunk),
                        _ => self.show_free_roam(ui, chunk_id),
                    }
                }

                ui.label(format!("Packets: {}", packets_count));
            });
        });
    }
}

impl Default for ChunkPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl EventGenerator for ChunkPanel {
    fn retrieve_events(&mut self) -> Option<HashMap<u8, Event>> {
        if self.events.is_empty() {
            return None;
        }

        let events = replace(&mut self.events, HashMap::with_capacity(1));
        Some(events)
    }
}
