use crate::dialog;
use crate::forza;
use crate::gui::*;
use eframe::{egui, epi};

use std::mem::take;
use std::{fs::File, io};

fn load_image(path: &str) -> io::Result<((usize, usize), Vec<egui::Color32>)> {
    let image = image::open(path).expect("Failed to load image").to_rgba8();
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
            forza::chunkify(
                self.socket.try_iter().filter(|p| {
                    !self.control_panel.want_next_race() || p.game_mode() == forza::GameMode::Race
                }),
                &mut self.chunks,
            );
            self.chunk_panel.selection = ChunkSelection(
                self.chunks.len() - 1,
                self.chunks.iter().last().map(|c| c.lap_count()),
            );
            self.last_selection = None;
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

    fn load_file(&mut self, path: &str) {
        match File::open(path).and_then(|mut f| forza::read_packets(&mut f)) {
            Ok(packets) => {
                forza::chunkify(packets.into_iter(), &mut self.chunks);
                self.last_selection = None;
            }
            Err(error) => {
                dialog::error_dialog(&format!("Failed to open {:}", &path), &error.to_string())
            }
        }
    }

    fn store_file(&self, path: &str) {
        if let Err(error) = File::create(path).and_then(|mut f| {
            self.chunks
                .iter()
                .try_for_each(|c| forza::write_packets(c.packets.iter(), &mut f))
        }) {
            dialog::error_dialog(
                &format!("Failed to write to {:}", &path),
                &error.to_string(),
            )
        }
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

        for file in &ctx.input().raw.dropped_files {
            if let Some(path) = file.path.as_ref().and_then(|p| p.to_str()) {
                self.load_file(path);
            }
        }

        self.control_panel.show(ctx);
        match take(&mut self.control_panel.action) {
            Some(ControlAction::Load(path)) => self.load_file(&path),
            Some(ControlAction::Save(path)) => self.store_file(&path),
            None => {}
        }

        self.chunk_panel.show(ctx, &self.chunks);

        if Some(self.chunk_panel.selection) != self.last_selection {
            self.update_chunk_selection();
        }

        self.map_panel.show(ctx);
    }
}
