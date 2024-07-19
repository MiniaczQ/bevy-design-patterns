use bevy::prelude::*;
use boilerplate::*;

mod boilerplate {
    use bevy::ecs::event::Event;

    use super::event_actor::ActorEvent;
    use super::event_emitter::EmitterEvent;
    
    #[derive(Event, Clone)]
    /// Glue for the different event types of the plugins
    pub struct GameEvent(pub &'static str);

    impl From<EmitterEvent> for GameEvent {
        fn from(other: EmitterEvent) -> Self {
            Self(other.0)
        }
    }

    impl From<GameEvent> for ActorEvent {
        fn from(other: GameEvent) -> Self {
            Self(other.0.len())
        }
    }
}

fn main() {
    App::new()
        .add_event::<GameEvent>()
        .add_plugins(DefaultPlugins)
        
        // Note you can comment out each plugin without breaking code
        .add_plugins(event_emitter::plugin::<GameEvent>)
        .add_plugins(event_actor::plugin::<GameEvent>)
        
        .run();
}

mod event_actor {
    // Note there is no import from the emitter
    use bevy::ecs::event::Event;
    use bevy::prelude::*;

    #[derive(Event)]
    pub struct ActorEvent(pub usize);

    pub fn plugin<E: Event + Clone + Into<ActorEvent>>(app: &mut App) {
        app.add_systems(Update, act::<E>);
    }

    fn act<E: Event + Clone + Into<ActorEvent>>(mut events_in: EventReader<E>) {
        for event in events_in.read() {
            // with this transformation we can act on all of the data of the event.
            let event: ActorEvent = (*event).clone().into();
            info!("Event: {}", event.0);
        }
    }
}

mod event_emitter {
    // Note there is no import from the actor
    use bevy::ecs::event::Event;
    use bevy::prelude::*;

    #[derive(Event)]
    pub struct EmitterEvent(pub &'static str);

    pub fn plugin<E: Event + From<EmitterEvent>>(app: &mut App) {
        app.add_event::<EmitterEvent>()
            .add_systems(PreUpdate, send::<E>);
    }

    fn send<E: Event + From<EmitterEvent>>(mut events_out: EventWriter<E>) {
        events_out.send(E::from(EmitterEvent("my event")));
    }
}
