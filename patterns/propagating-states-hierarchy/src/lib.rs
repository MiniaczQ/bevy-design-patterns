use bevy::prelude::*;

pub struct PropagatingStatesHierarchyPlugin;

impl Plugin for PropagatingStatesHierarchyPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MajorState>();
        app.add_state::<MinorState>();
        app.add_systems(OnEnter(MajorState::B), update_state(MinorState::C));
        app.add_systems(OnExit(MajorState::B), update_state(MinorState::None));
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum MajorState {
    #[default]
    A,
    B,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum MinorState {
    #[default]
    None,
    C,
    D,
}

pub fn update_state<T: States>(state: T) -> impl Fn(ResMut<NextState<T>>) {
    move |mut next_state: ResMut<NextState<T>>| {
        next_state.set(state.clone());
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{update_state, PropagatingStatesHierarchyPlugin, MajorState, MinorState};

    #[test]
    fn default_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(PropagatingStatesHierarchyPlugin);

        assert_eq!(*app.world.resource::<State<MajorState>>(), MajorState::A);
        assert_eq!(*app.world.resource::<State<MinorState>>(), MinorState::None);
    }

    #[test]
    fn change_major_state_to_b() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(PropagatingStatesHierarchyPlugin);
        app.add_systems(Update, update_state(MajorState::B).run_if(run_once()));

        for _ in 0..3 {
            app.update();
        }

        assert_eq!(*app.world.resource::<State<MajorState>>(), MajorState::B);
        assert_eq!(*app.world.resource::<State<MinorState>>(), MinorState::C);
    }

    #[test]
    fn change_major_state_to_b_then_a() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(PropagatingStatesHierarchyPlugin);
        app.add_systems(Update, update_state(MajorState::B).run_if(run_once()));
        app.add_systems(OnEnter(MajorState::B), update_state(MajorState::A));

        for _ in 0..4 {
            app.update();
        }

        assert_eq!(*app.world.resource::<State<MajorState>>(), MajorState::A);
        assert_eq!(*app.world.resource::<State<MinorState>>(), MinorState::None);
    }
}
