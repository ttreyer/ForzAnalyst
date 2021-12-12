use crate::egui_backend::egui;
use crate::forza::{self, chunkify, write_packets};
use crate::gui::{chunk_panel::*, control_panel::*, map_panel::*};

use egui::CtxRef;
use egui::TextureId;

use std::{fs::File, io};
use tinyfiledialogs::{message_box_ok, MessageBoxIcon};

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
            chunks: Self::load_file("race.ftm").unwrap(),
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
        match &self.control_panel.action {
            Some(ControlAction::Load(path)) => match Self::load_file(&path) {
                Ok(mut new_chunks) => self.chunks.append(&mut new_chunks),
                Err(error) => message_box_ok(
                    &format!("Failed to open {:}", &path),
                    &error.to_string(),
                    MessageBoxIcon::Error,
                ),
            },
            Some(ControlAction::Save(path)) => {
                if let Err(error) = Self::store_file(&path, &self.chunks) {
                    message_box_ok(
                        &format!("Failed to write to {:}", &path),
                        &error.to_string(),
                        MessageBoxIcon::Error,
                    )
                }
            }
            None => {}
        }

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

    fn load_file(path: &str) -> io::Result<forza::Chunks> {
        let mut file = File::open(path)?;
        let packets = forza::read_packets(&mut file)?;
        let mut chunks = forza::Chunks::new();
        chunkify(packets.into_iter(), &mut chunks);
        Ok(chunks)
    }

    fn store_file(path: &str, chunks: &forza::Chunks) -> io::Result<()> {
        let mut file = File::create(path)?;
        for chunk in chunks {
            write_packets(chunk.packets.iter(), &mut file)?;
        }
        Ok(())
    }
}
