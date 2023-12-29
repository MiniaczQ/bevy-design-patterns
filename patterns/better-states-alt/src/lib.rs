mod states;

use bevy::prelude::*;

// The goal is to integrate this into `States`.
pub trait StateComparator: States {
    /// Substate-free
    type Ssf: States;

    /// Copy into a substate-free type variant.
    fn ssf(&self) -> Self::Ssf;

    /// Run transition schedules if necessary.
    fn transition(&self, next: &Self, world: &mut World) {
        // Return early if state didn't change.
        if self.eq(next) {
            return;
        }

        // Check whether the change is in this state or some sub-state.
        // Do this by comparing this state without the `#[substate]` fields.
        let self_just = self.ssf();
        let next_just = next.ssf();

        if self_just == next_just {
            // Change in sub-states.
            self.transition_substates(next, world);
        } else {
            // Change in this state.
            self.exit_substates(world);
            world
                .try_run_schedule(OnTransition {
                    from: self_just,
                    to: next_just,
                })
                .ok();
            next.enter_substates(world);
        }
    }

    /// Run `transition` for all `#[substate]` fields.
    fn transition_substates(&self, next: &Self, world: &mut World);

    /// Run `OnExit` for all substates.
    fn exit_substates(&self, world: &mut World);

    /// Run `OnEnter` for all substates.
    fn enter_substates(&self, world: &mut World);
}

pub trait AddStateV2 {
    fn add_state_v2<S: States + StateComparator>(&mut self) -> &mut Self;
}

pub fn run_enter_schedule<S: States + StateComparator>(world: &mut World) {
    world
        .try_run_schedule(OnEnter(world.resource::<State<S>>().get().ssf()))
        .ok();
}

pub fn apply_state_transition<S: States + StateComparator>(world: &mut World) {
    let mut next_state_resource = world.resource_mut::<NextState<S>>();
    if let Some(entered) = next_state_resource.bypass_change_detection().0.take() {
        next_state_resource.set_changed();
        let exited;
        {
            let mut exited_res = world.resource_mut::<State<S>>();
            exited = exited_res.get().clone();
            *exited_res = State::new(entered.clone());
        }
        exited.transition(&entered, world);
    }
}

impl AddStateV2 for App {
    fn add_state_v2<S: States + StateComparator>(&mut self) -> &mut Self {
        self.init_resource::<State<S>>()
            .init_resource::<NextState<S>>()
            .add_systems(
                StateTransition,
                (
                    run_enter_schedule::<S>.run_if(run_once()),
                    apply_state_transition::<S>,
                )
                    .chain(),
            );
        self
    }
}
