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
    packet_panel: PacketPanel,
    chunks: forza::Chunks,
    socket: forza::Socket,
    last_selection: Option<ChunkSelection>,
}

impl App {
    pub fn process(&mut self) {
        if !self.control_panel.is_record() {
            // Clear non-recorded packets
            self.socket.try_iter().last();
        } else {
            // Filter-out non-race packets if we only record race data
            let wanted_packets = self.socket.try_iter().filter(|p| {
                !self.control_panel.want_next_race() || p.game_mode() == forza::GameMode::Race
            });
            forza::chunkify(wanted_packets, &mut self.chunks);

            // Update UI to follow player
            self.last_selection = None;
            self.chunk_panel.selection = ChunkSelection(
                self.chunks.len() - 1,
                self.chunks.iter().last().map(|c| c.lap_count()),
            );
        }

        if let Some(chunk_selection) = take(&mut self.chunk_panel.trash_chunk) {
            self.last_selection = None;
            match chunk_selection {
                ChunkSelection(chunk_id, None) => {
                    self.remove_chunk(chunk_id);
                }
                ChunkSelection(chunk_id, Some(lap_num)) => {
                    let chunk = self.chunks.iter_mut().nth(chunk_id).unwrap();
                    chunk.remove_lap(lap_num);
                    if chunk.packets.is_empty() {
                        self.remove_chunk(chunk_id);
                    }
                }
            }
        }
    }

    fn remove_chunk(&mut self, id: ChunkID) {
        let mut split_list = self.chunks.split_off(id);
        split_list.pop_front();
        self.chunks.append(&mut split_list);
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
            self.last_selection = Some(self.chunk_panel.selection);
            self.map_panel
                .set_packets(self.chunk_panel.selected_packets(&self.chunks));
        }

        let selected_packets = self.chunk_panel.selected_packets(&self.chunks);
        let hovered_packet = self.map_panel.hovered_packet(selected_packets);
        self.packet_panel.show(ctx, hovered_packet);

        self.map_panel.show(ctx);
    }
}
