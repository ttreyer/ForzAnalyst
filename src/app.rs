use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{chunkify, read_packets, Chunks, ForzaChunk, ForzaGameMode};
use crate::{
    forza::forza_packet::ForzaSocket,
    gui::{chunk_panel::ChunkPanel, control_panel::ControlPanel, tty_plot_panel::TtyPlotPanel},
};

use egui::plot::Plot;
use egui::Align2;
use egui::CtxRef;
use egui_sdl2_gl::egui::plot::{PlotImage, Value};
use egui_sdl2_gl::egui::TextureId;

pub struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    tty_plot_panel: TtyPlotPanel,
    chunks: Chunks,
    socket: ForzaSocket,
    map: TextureId,
}

impl App {
    pub fn new(addr: &str, map: TextureId) -> Self {
        Self {
            control_panel: ControlPanel::new(),
            chunk_panel: ChunkPanel::new(),
            tty_plot_panel: TtyPlotPanel::new(),
            chunks: chunkify(read_packets(&mut std::fs::File::open("race.ftm").unwrap())),
            socket: ForzaSocket::new(addr),
            map,
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

        egui::Window::new("Map")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .show(ctx, |ui| {
                let image_plot =
                    PlotImage::new(self.map, Value { x: 0f64, y: 0f64 }, [2400.0, 1600.0]);
                let plot = Plot::new("map")
                    .allow_drag(true)
                    .allow_zoom(true)
                    .data_aspect(1.0)
                    .image(image_plot);
                ui.add(plot);
            });

        let selected_chunk = self
            .chunks
            .iter()
            .nth(self.chunk_panel.selection.chunk_id)
            .unwrap();
        self.tty_plot_panel.show(ctx, &selected_chunk.packets[..]);
    }
}
