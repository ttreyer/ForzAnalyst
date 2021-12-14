use crate::egui_backend::egui;
use crate::forza::{self, chunkify};
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
            chunks: Self::load_file("goliath_zstd.ftm").unwrap(),
            socket: forza::Socket::new(addr),
        }
    }

    pub fn process(&mut self) {
        if self.control_panel.is_record() {
            chunkify(self.socket.try_iter(), &mut self.chunks);
        } else {
            self.socket.try_iter().last();
        }

        if let Some(chunk_selection) = self.chunk_panel.trash_chunk {
            match chunk_selection {
                ChunkSelection(chunk_id, None) => {
                    Self::remove_chunk(&mut self.chunks, chunk_id);
                }
                ChunkSelection(chunk_id, Some(lap_id)) => {
                    let chunk = self.chunks.iter_mut().nth(chunk_id);

                    if let Some(chunk) = chunk {
                        chunk.lap_keep[lap_id as usize] = false;
                        let keep = chunk.lap_keep.iter().fold(false, |accum, i| accum | i);

                        if !keep {
                            Self::remove_chunk(&mut self.chunks, chunk_id)
                        }
                    }
                }
            }

            self.chunk_panel.trash_chunk = None;
        }
    }

    fn remove_chunk(chunks: &mut forza::Chunks, id: ChunkID) {
        let mut split_list = chunks.split_off(id);
        split_list.pop_front();
        chunks.append(&mut split_list);
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
        forza::read_chunks(&mut File::open(path)?)
    }

    fn store_file(path: &str, chunks: &forza::Chunks) -> io::Result<()> {
        forza::write_chunks(chunks.iter(), &mut File::create(path)?)
    }
}
