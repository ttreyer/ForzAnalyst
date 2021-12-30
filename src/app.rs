use crate::dialog;
use crate::event::Event;
use crate::event::EventGenerator;
use crate::process_events;
use crate::forza;
use crate::forza::chunk::ChunkSelector;
use crate::gui::*;
use eframe::{egui, epi};

use std::collections::HashMap;
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
    last_selector: Option<ChunkSelector>,
}

impl App {
    pub fn process(&mut self) {
        if !self.control_panel.is_record() {
            // Clear non-recorded packets
            self.socket.try_iter().last();
        } else {
            // Filter-out non-race packets if we only record race data
            let wanted_packets = self.socket.try_iter();//.filter(|p| {
            //    !self.control_panel.want_next_race() || p.game_mode() == forza::GameMode::Race
            //});
            self.chunks.chunkify(wanted_packets);

            self.last_selector = None;
            self.chunk_panel.set_selection(self.chunks.last_chunk_selector());
        }
    }

    fn load_file(&mut self, path: &str) {
        match File::open(path).and_then(|mut f| forza::read_packets(&mut f)) {
            Ok(packets) => {
                self.chunks.chunkify(packets.into_iter());
            }
            Err(error) => {
                dialog::error_dialog(&format!("Failed to open {:}", &path), &error.to_string())
            }
        }
    }

    fn save_file(&self, path: &str) {
        if let Err(error) = File::create(path).and_then(|mut f| {
            self.chunks
                .list()
                .iter()
                .try_for_each(|c| forza::write_packets(c.packets.iter(), &mut f))
        }) {
            dialog::error_dialog(
                &format!("Failed to write to {:}", &path),
                &error.to_string(),
            )
        }
    }

    fn process_events(&mut self, events: Option<HashMap<u8, Event>>) {
        if let Some(events) = events {
            for (_, event) in events {
                match event {
                    Event::ControlPanelEvent(event) => match event {
                        ControlPanelEvent::Load(path) => self.load_file(&path),
                        ControlPanelEvent::Save(path) => self.save_file(&path),
                    },
                    Event::ChunkPanelEvent(event) => match event {
                        ChunkPanelEvent::ChangeSelection(chunk_selector) => {
                            if Some(chunk_selector) != self.last_selector {
                                self.last_selector = Some(chunk_selector);
                                self.map_panel
                                    .set_packets(self.chunk_panel.selected_packets(&self.chunks));
                            }
                        },
                        ChunkPanelEvent::RemoveChunk(chunk_selector) => {
                            self.chunks.remove_chunk(&chunk_selector);

                            // Force follow last chunk
                            self.last_selector = None;
                            self.chunk_panel.set_selection(self.chunks.last_chunk_selector());
                            self.map_panel
                                .set_packets(self.chunk_panel.selected_packets(&self.chunks));
                        },
                    },
                }
            }
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
        process_events!(self, control_panel);

        self.chunk_panel.show(ctx, &self.chunks);
        process_events!(self, chunk_panel);

        let selected_packets = self.chunk_panel.selected_packets(&self.chunks);
        let hovered_packet = self.map_panel.hovered_packet(selected_packets);
        self.packet_panel.show(ctx, hovered_packet);

        self.map_panel.show(ctx);
    }
}
