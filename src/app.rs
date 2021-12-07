use crate::{gui::{control_panel::ControlPanel, chunk_panel::ChunkPanel}, forza::forza_packet::ForzaSocket};

struct App {
    control_panel: ControlPanel,
    chunk_panel: ChunkPanel,
    socket: ForzaSocket,
}

impl App {
    pub fn new() -> Self {
        Self {
            control_panel: ControlPanel::new(),
            chunk_panel: ChunkPanel::new(),
            socket: ForzaSocket::new("0.0.0.0:7024"),
        }
    }

    pub fn main() {

    }
}