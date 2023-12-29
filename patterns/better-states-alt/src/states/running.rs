use bevy::prelude::*;

use crate::StateComparator;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum RunningState {
    #[default]
    Running,
    Paused,
}

// MACRO GENERATED
// No `#[substate]` fields, we can use a type alias.
// If we use a different trait for SSF, we cannot use type aliases.
pub type SsfRunningState = RunningState;

// MACRO GENERATED
impl StateComparator for RunningState {
    type Ssf = SsfRunningState;

    #[allow(clippy::clone_on_copy)] // `Copy` is not required by `States`.
    fn ssf(&self) -> Self::Ssf {
        self.clone()
    }

    #[allow(clippy::match_single_binding, unused_variables)]
    fn transition_substates(&self, next: &Self, world: &mut World) {
        match (self, next) {
            // No `#[substate]` fields for any variant.
            // Guaranteed by the `transition` implementation.
            _ => unreachable!("Enum variants are not the same"),
        }
    }

    #[allow(unused_variables)]
    fn exit_substates(&self, world: &mut World) {
        world.try_run_schedule(OnExit(self.ssf())).ok();
    }

    #[allow(unused_variables)]
    fn enter_substates(&self, world: &mut World) {
        world.try_run_schedule(OnEnter(self.ssf())).ok();
    }
}
