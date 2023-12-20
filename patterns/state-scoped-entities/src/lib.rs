use bevy::prelude::*;

pub struct StateScopedEntitiesPlugin;

impl Plugin for StateScopedEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MyState>();
        app.add_systems(Update, despawn_scoped::<MyState>);
    }
}

#[derive(Component)]
pub struct Scoped<T: States>(pub T);

pub fn despawn_scoped<T: States + PartialEq<T>>(
    mut commands: Commands,
    app_state: Res<State<T>>,
    query: Query<(Entity, &Scoped<T>)>,
) {
    if !app_state.is_changed() {
        return;
    }
    for (entity, on_state) in query.iter() {
        if &on_state.0 != app_state.get() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum MyState {
    #[default]
    A,
    B,
}

pub fn update_state<T: States>(state: T) -> impl Fn(ResMut<NextState<T>>) {
    move |mut next_state: ResMut<NextState<T>>| {
        next_state.set(state.clone());
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{update_state, MyState, Scoped, StateScopedEntitiesPlugin};

    #[test]
    fn no_state_change() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(StateScopedEntitiesPlugin);
        let id = app.world.spawn(Scoped(MyState::A)).id();

        assert!(app.world.get_entity(id).is_some());
    }

    #[test]
    fn state_changed_entity_removed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(StateScopedEntitiesPlugin);
        let id = app.world.spawn(Scoped(MyState::A)).id();
        app.add_systems(Update, update_state(MyState::B).run_if(run_once()));

        app.update();
        app.update();

        assert!(app.world.get_entity(id).is_none());
    }
}
