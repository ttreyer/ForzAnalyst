use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{Chunks, ForzaChunk, ForzaGameMode};
use crate::gui::control_panel;
use crate::{
    forza::forza_packet::ForzaSocket,
    gui::{chunk_panel::ChunkPanel, control_panel::ControlPanel},
};

use egui::CtxRef;

pub struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    chunks: Chunks,
    socket: ForzaSocket,
}

impl App {
    pub fn new(addr: &str) -> Self {
        Self {
            control_panel: ControlPanel::new(),
            chunk_panel: ChunkPanel::new(),
            chunks: LinkedList::from([ForzaChunk::new()]),
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
                        _ => self.chunks.push_back(ForzaChunk::new()),
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
    }
}
