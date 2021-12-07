use std::collections::LinkedList;

use crate::egui_backend::egui;
use crate::forza::forza_packet::{Chunks, ForzaChunk};
use crate::gui::control_panel;
use crate::{
    forza::forza_packet::ForzaSocket,
    gui::{chunk_panel::ChunkPanel, control_panel::ControlPanel},
};

use egui::CtxRef;

struct App {
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
            socket: ForzaSocket::new("0.0.0.0:7024"),
        }
    }

    pub fn process() {}

    pub fn show(&mut self, ctx: &CtxRef) {
        self.control_panel.show(ctx);

        self.chunk_panel.show(ctx, &self.chunks);
    }
}
