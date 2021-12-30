use std::collections::HashMap;

use crate::forza::chunk::ChunksEvent;
use crate::gui::chunk_panel::ChunkPanelEvent;
use crate::gui::control_panel::ControlPanelEvent;

pub enum Event {
    ControlPanelEvent(ControlPanelEvent),
    ChunksEvent(ChunksEvent),
    ChunkPanelEvent(ChunkPanelEvent),
}

pub trait EventGenerator {
    fn retrieve_events(&mut self) -> Option<HashMap<u8, Event>>;

    fn drop_events(&mut self) {
        self.retrieve_events();
    }
}

#[macro_export]
macro_rules! process_events {
    ( $self:ident, $event_generator:ident ) => {
        let events = $self.$event_generator.retrieve_events();
        $self.process_events(events);
    };
}
