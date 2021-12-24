use crate::forza::{self, chunkify};
use crate::gui::{chunk_panel::*, control_panel::*, map_panel::*};
use eframe::{egui, epi};

use std::{fs::File, io};
use tinyfiledialogs::{message_box_ok, MessageBoxIcon};

fn load_image(path: &str) -> io::Result<((usize, usize), Vec<egui::Color32>)> {
    let file = std::io::BufReader::new(File::open(path)?);
    let image = image::load(file, image::ImageFormat::Jpeg)
        .expect("Failed to load image")
        .to_rgba8();
    let size = (image.width() as usize, image.height() as usize);
    let pixels: Vec<_> = image
        .pixels()
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    Ok((size, pixels))
}

#[derive(Default)]
pub struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    map_panel: MapPanel,
    chunks: forza::Chunks,
    socket: forza::Socket,
    last_selection: Option<ChunkSelection>,
}

impl App {
    pub fn process(&mut self) {
        if self.control_panel.is_record() {
            chunkify(self.socket.try_iter(), &mut self.chunks);
        } else {
            self.socket.try_iter().last();
        }

        if let Some(chunk_selection) = &self.chunk_panel.trash_chunk {
            match *chunk_selection {
                ChunkSelection(chunk_id, None) => {
                    Self::remove_chunk(&mut self.chunks, chunk_id);
                }
                ChunkSelection(chunk_id, Some(lap_num)) => {
                    let chunk = self.chunks.iter_mut().nth(chunk_id).unwrap();
                    chunk.remove_lap(lap_num);
                    if chunk.packets.is_empty() {
                        Self::remove_chunk(&mut self.chunks, chunk_id);
                    }
                }
            }
            self.last_selection = None;
        }
    }

    fn remove_chunk(chunks: &mut forza::Chunks, id: ChunkID) {
        let mut split_list = chunks.split_off(id);
        split_list.pop_front();
        chunks.append(&mut split_list);
    }

    fn load_file(path: &str) -> io::Result<forza::Chunks> {
        forza::read_chunks(&mut File::open(path)?)
    }

    fn store_file(path: &str, chunks: &forza::Chunks) -> io::Result<()> {
        forza::write_chunks(chunks.iter(), &mut File::create(path)?)
    }

    fn update_chunk_selection(&mut self) {
        self.last_selection = Some(self.chunk_panel.selection);
        let ChunkSelection(chunk_id, lap_id) = self.chunk_panel.selection;

        let packets;
        match (self.chunks.iter().nth(chunk_id), lap_id) {
            (Some(selected_chunk), Some(lap)) => packets = selected_chunk.lap_packets(lap),
            (Some(selected_chunk), None) => packets = &selected_chunk.packets,
            (None, _) => packets = &[],
        }
        self.map_panel.set_packets(packets);
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "ForzAnalyst"
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let (size, pixels) = load_image("fh5_map.jpg").expect("Failed to load image");
        let map = frame
            .tex_allocator()
            .alloc_srgba_premultiplied(size, &pixels);
        self.map_panel
            .set_image(egui::Vec2::new(size.0 as f32, size.1 as f32), map);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.process();

        self.control_panel.show(ctx);
        match &self.control_panel.action {
            Some(ControlAction::Load(path)) => match Self::load_file(&path) {
                Ok(mut new_chunks) => {
                    self.chunks.append(&mut new_chunks);
                    self.last_selection = None
                }
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

        if Some(self.chunk_panel.selection) != self.last_selection {
            self.update_chunk_selection();
        }

        self.map_panel.show(ctx);
    }
}
