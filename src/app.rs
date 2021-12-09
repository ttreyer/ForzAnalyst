use crate::egui_backend::egui;
use crate::forza::forza_packet::*;
use crate::gui::chunk_panel::ChunkSelection;
use crate::gui::{chunk_panel::ChunkPanel, control_panel::ControlPanel, map_panel::MapPanel};

use egui::CtxRef;
use egui::TextureId;

pub struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    map_panel: MapPanel,
    chunks: Chunks,
    socket: ForzaSocket,
}

impl App {
    pub fn new(addr: &str, map: TextureId) -> Self {
        Self {
            control_panel: ControlPanel::new(),
            chunk_panel: ChunkPanel::new(),
            map_panel: MapPanel::new(map),
            chunks: chunkify(read_packets(&mut std::fs::File::open("race.ftm").unwrap())),
            socket: ForzaSocket::new(addr),
        }
    }

    pub fn process(&mut self) {
        let data = self.socket.try_iter();

        if self.control_panel.is_record() {
            for p in data {
                if p.current_race_time == 0.0 {
                    match (p.game_mode(), self.chunks.back().unwrap().game_mode()) {
                        (ForzaGameMode::FreeRoam, ForzaGameMode::FreeRoam) => {}
                        _ => {
                            self.chunks.back_mut().map(|c| c.finalize());
                            if self.chunks.back().map(|c| c.is_empty()).unwrap_or(true) {
                                self.chunks.push_back(ForzaChunk::new())
                            }
                        }
                    }
                }

                self.chunks.back_mut().unwrap().push(p);
            }
        } else {
            let _ = data.last();
        }
    }

    pub fn show(&mut self, ctx: &CtxRef) {
        self.control_panel.show(ctx);

        self.chunk_panel.show(ctx, &self.chunks);

        let ChunkSelection(chunk_id, _) = self.chunk_panel.selection;
        let selected_chunk = self.chunks.iter().nth(chunk_id).unwrap();
        self.map_panel.show(ctx, &selected_chunk.packets);
    }
}
