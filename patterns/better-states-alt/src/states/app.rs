use bevy::prelude::*;

use crate::StateComparator;

use super::gameplay::GameplayState;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    // Tuple struct variant:
    Gameplay(
        // #[substate]
        GameplayState,
    ),
    // OR struct variant:
    // Gameplay {
    //  #[substate]
    //  inner: GameplayState
    // }
}

// MACRO GENERATED
// Copy all fields without `#[substate]`
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, States)]
pub enum SsfAppState {
    #[default]
    MainMenu,
    Gameplay, // NOTE: Brackets collapsed, because no fields left
}

// MACRO GENERATED
impl StateComparator for AppState {
    type Ssf = SsfAppState;

    fn ssf(&self) -> Self::Ssf {
        match self {
            AppState::MainMenu => SsfAppState::MainMenu,
            AppState::Gameplay(_) => SsfAppState::Gameplay,
        }
    }

    fn transition_substates(&self, next: &Self, world: &mut World) {
        match (self, next) {
            // No `MainMenu` branch.
            // Variants with no `#[substate]` fields shouldn't pass the original `eq` check.
            (AppState::Gameplay(s1), AppState::Gameplay(s2)) => s1.transition(s2, world),
            // Guaranteed by the `transition` implementation.
            _ => unreachable!("Enum variants are not the same"),
        }
    }

    #[allow(clippy::single_match)]
    fn exit_substates(&self, world: &mut World) {
        match self {
            AppState::Gameplay(s) => s.exit_substates(world),
            _ => {}
        }
        world.try_run_schedule(OnExit(self.ssf())).ok();
    }

    #[allow(clippy::single_match)]
    fn enter_substates(&self, world: &mut World) {
        world.try_run_schedule(OnEnter(self.ssf())).ok();
        match self {
            AppState::Gameplay(s) => s.enter_substates(world),
            _ => {}
        }
    }
}
