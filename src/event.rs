pub type EventId<EventTypes> = std::mem::Discriminant<EventTypes>;
pub type Events<EventTypes> = std::collections::HashMap<EventId<EventTypes>, EventTypes>;

pub trait EventGenerator<EventTypes> {
    fn gen_event(&mut self, event: EventTypes) {
        self.events().insert(std::mem::discriminant(&event), event);
    }

    fn events(&mut self) -> &mut Events<EventTypes>;
}

pub trait EventHandler<EventTypes> {
    fn handle(&mut self, event: EventTypes);
    fn generator(&mut self) -> &mut dyn EventGenerator<EventTypes>;

    fn handle_events(&mut self) {
        let generator = self.generator();
        let events = std::mem::take(generator.events());
        events.into_values().for_each(|e| self.handle(e));
    }
}

#[macro_export]
macro_rules! process_events {
    ( $self:ident, $event_types:ty ) => {
        EventHandler::<$event_types>::handle_events($self);
    };
}
