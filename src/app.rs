use crate::egui_backend::egui;
use crate::forza::{self, chunkify};
use crate::gui::chunk_panel::ChunkSelection;
use crate::gui::{chunk_panel::ChunkPanel, control_panel::ControlPanel, map_panel::MapPanel};

use egui::CtxRef;
use egui::TextureId;

use std::{fs::File, io, path::Path};

pub struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    map_panel: MapPanel,
    chunks: forza::Chunks,
    socket: forza::Socket,
}

impl App {
    pub fn new(addr: &str, map: TextureId) -> Self {
        Self {
            control_panel: ControlPanel::new(),
            chunk_panel: ChunkPanel::new(),
            map_panel: MapPanel::new(map),
            chunks: Self::load_file(Path::new("race.ftm")).unwrap(),
            socket: forza::Socket::new(addr),
        }
    }

    pub fn process(&mut self) {
        let data = self.socket.try_iter();

        if self.control_panel.is_record() {
            for p in data {
                if p.current_race_time == 0.0 {
                    match (p.game_mode(), self.chunks.back().unwrap().game_mode()) {
                        (forza::GameMode::FreeRoam, forza::GameMode::FreeRoam) => {}
                        _ => {
                            self.chunks.back_mut().map(|c| c.finalize());
                            if self.chunks.back().map(|c| c.is_empty()).unwrap_or(true) {
                                self.chunks.push_back(forza::Chunk::new())
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

        let ChunkSelection(chunk_id, lap_id) = self.chunk_panel.selection;
        let selected_chunk = self.chunks.iter().nth(chunk_id).unwrap();
        match lap_id {
            Some(lap) => self.map_panel.show(ctx, selected_chunk.lap_packets(lap)),
            None => self.map_panel.show(ctx, selected_chunk.packets.iter()),
        };
    }

    fn load_file(path: &Path) -> io::Result<forza::Chunks> {
        let mut file = File::open(path)?;
        let packets = forza::read_packets(&mut file)?;
        Ok(chunkify(packets))
    }
}
