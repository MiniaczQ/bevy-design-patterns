use bevy::prelude::*;

use crate::StateComparator;

use super::running::RunningState;

// Struct:
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub struct GameplayState {
    // #[substate]
    pub running: RunningState,
    pub difficulty: u32,
}
// OR tuple struct:
// struct GameplayState(#[substate] pub RunningState, u32)

// MACRO GENERATED
// Copy all fields without `#[substate]`
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub struct SsfGameplayState {
    pub difficulty: u32,
}

// MACRO GENERATED
impl StateComparator for GameplayState {
    type Ssf = SsfGameplayState;

    fn ssf(&self) -> Self::Ssf {
        SsfGameplayState {
            difficulty: self.difficulty,
        }
    }

    fn transition_substates(&self, next: &Self, world: &mut World) {
        self.running.transition(&next.running, world);
    }

    fn exit_substates(&self, world: &mut World) {
        self.running.exit_substates(world);
        world.try_run_schedule(OnExit(self.ssf())).ok();
    }

    fn enter_substates(&self, world: &mut World) {
        world.try_run_schedule(OnEnter(self.ssf())).ok();
        self.running.enter_substates(world);
    }
}
