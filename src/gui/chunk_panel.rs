use crate::egui_backend::egui;
use crate::forza;

type ChunkID = usize;
type LapID = Option<u16>;

#[derive(PartialEq)]
pub struct ChunkSelection(pub ChunkID, pub LapID);

pub struct ChunkPanel {
    pub selection: ChunkSelection,
}

impl ChunkPanel {
    pub fn new() -> Self {
        Self {
            selection: ChunkSelection(0, None),
        }
    }

    fn select(&mut self, chunk_id: ChunkID, lap_id: LapID) {
        self.selection = ChunkSelection(chunk_id, lap_id)
    }

    fn is_selected(&self, chunk_id: ChunkID, lap_id: LapID) -> bool {
        ChunkSelection(chunk_id, lap_id) == self.selection
    }

    fn show_free_roam(&mut self, ui: &mut egui::Ui, chunk_id: ChunkID) -> egui::Response {
        ui.selectable_label(self.is_selected(chunk_id, None), "Free Roam")
    }

    fn show_race(
        &mut self,
        ui: &mut egui::Ui,
        chunk_id: usize,
        chunk: &forza::Chunk,
    ) -> egui::Response {
        egui::CollapsingHeader::new("Race")
            .id_source(chunk_id)
            .selectable(true)
            .selected(self.is_selected(chunk_id, None))
            .default_open(true)
            .show(ui, |ui| {
                for lap_id in 0..=chunk.lap_count() {
                    if ui
                        .selectable_label(
                            self.is_selected(chunk_id, Some(lap_id)),
                            format!("Lap {}", lap_id),
                        )
                        .clicked()
                    {
                        self.select(chunk_id, Some(lap_id))
                    }
                }
            })
            .header_response
    }

    pub fn show(&mut self, ctx: &egui::CtxRef, chunks: &forza::Chunks) {
        egui::Window::new("Chunk").show(ctx, |ui| {
            let mut packets_count = 0usize;

            for (chunk_id, chunk) in chunks.iter().enumerate() {
                packets_count += chunk.packets.len();
                match chunk.game_mode() {
                    forza::GameMode::FreeRoam => {
                        if self.show_free_roam(ui, chunk_id).clicked() {
                            self.select(chunk_id, None)
                        }
                    }
                    forza::GameMode::Race => {
                        if self.show_race(ui, chunk_id, &chunk).clicked() {
                            self.select(chunk_id, None);
                        }
                    }
                }
            }

            ui.label(format!("Packets: {}", packets_count));
        });
    }
}
