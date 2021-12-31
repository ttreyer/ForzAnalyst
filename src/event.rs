pub type EventId = u8;
pub type Events<EventTypes> = std::collections::HashMap<u8, EventTypes>;

pub trait EventGenerator<EventTypes> {
    fn retrieve_events(&mut self) -> Events<EventTypes>;

    fn drop_events(&mut self) {
        self.retrieve_events();
    }
}

pub trait EventHandler<EventTypes> {
    fn process_event(&mut self, event: EventTypes);

    fn process_events(&mut self, events: Events<EventTypes>) {
        events.into_values().for_each(|e| self.process_event(e));
    }
}

#[macro_export]
macro_rules! process_events {
    ( $self:ident, $event_generator:ident ) => {
        let events = $self.$event_generator.retrieve_events();
        $self.process_events(events);
    };
}
