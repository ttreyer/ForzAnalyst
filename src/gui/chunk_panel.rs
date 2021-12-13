use crate::egui_backend::egui;
use crate::forza;

pub type ChunkID = usize;
pub type LapID = Option<u16>;

#[derive(PartialEq, Clone, Copy)]
pub struct ChunkSelection(pub ChunkID, pub LapID);

pub struct ChunkPanel {
    pub selection: ChunkSelection,
    pub trash_chunk: Option<ChunkSelection>,
}

impl ChunkPanel {
    pub fn new() -> Self {
        Self {
            selection: ChunkSelection(0, None),
            trash_chunk: None,
        }
    }

    fn select(&mut self, chunk_id: ChunkID, lap_id: LapID) {
        self.selection = ChunkSelection(chunk_id, lap_id)
    }

    fn is_selected(&self, chunk_id: ChunkID, lap_id: LapID) -> bool {
        ChunkSelection(chunk_id, lap_id) == self.selection
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
                for lap_id in 0..chunk.lap_count() {
                    if !chunk.lap_keep[lap_id as usize] {
                        continue;
                    }

                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                self.is_selected(chunk_id, Some(lap_id)),
                                format!("Lap {}", lap_id),
                            )
                            .clicked()
                        {
                            self.select(chunk_id, Some(lap_id))
                        }

                        if ui.button("🗑").clicked() {
                            self.trash_chunk(chunk_id, Some(lap_id));
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
    }
}
