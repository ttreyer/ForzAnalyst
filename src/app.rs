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
        if self.control_panel.is_record() {
            chunkify(self.socket.try_iter(), &mut self.chunks);
        } else {
            self.socket.try_iter().last();
        }
    }

    pub fn show(&mut self, ctx: &CtxRef) {
        self.control_panel.show(ctx);

        self.chunk_panel.show(ctx, &self.chunks);

        let ChunkSelection(chunk_id, lap_id) = self.chunk_panel.selection;
        if let Some(selected_chunk) = self.chunks.iter().nth(chunk_id) {
            match lap_id {
                Some(lap) => self.map_panel.show(ctx, selected_chunk.lap_packets(lap)),
                None => self.map_panel.show(ctx, &selected_chunk.packets),
            }
        } else {
            self.map_panel.show(ctx, &[]);
        }
    }

    fn load_file(path: &Path) -> io::Result<forza::Chunks> {
        let mut file = File::open(path)?;
        let packets = forza::read_packets(&mut file)?;
        let mut chunks = forza::Chunks::new();
        chunkify(packets.into_iter(), &mut chunks);
        Ok(chunks)
    }
}
