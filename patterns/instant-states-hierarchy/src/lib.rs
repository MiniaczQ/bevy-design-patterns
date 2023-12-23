use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum SubState<T: States> {
    #[default]
    Inactive,
    Active(T),
}

pub fn run_enter_schedule<S: States>(world: &mut World) {
    let mut substate = world.resource_mut::<State<SubState<S>>>();
    *substate = State::new(SubState::Active(S::default()));
    world.try_run_schedule(OnEnter(S::default())).ok();
}

pub fn change_state<S: States>(state: S) -> impl Fn(ResMut<NextState<S>>) {
    move |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    }
}

pub fn apply_on_transition<S: States>(world: &mut World) {
    let mut next_state_resource = world.resource_mut::<NextState<SubState<S>>>();
    let Some(entered) = next_state_resource.bypass_change_detection().0.take() else {
        return;
    };
    let mut state_resource = world.resource_mut::<State<SubState<S>>>();
    if *state_resource == entered {
        return;
    }
    // TODO: use `mem::replace` here
    let exited = state_resource.get().clone();
    *state_resource = State::new(entered.clone());

    match (exited, entered) {
        (SubState::Active(exited), SubState::Inactive) => {
            world.try_run_schedule(OnExit(exited)).ok();
        }
        (SubState::Active(exited), SubState::Active(entered)) => {
            world.try_run_schedule(OnExit(exited.clone())).ok();
            world
                .try_run_schedule(OnTransition {
                    from: exited,
                    to: entered.clone(),
                })
                .ok();
            world.try_run_schedule(OnEnter(entered)).ok();
        }
        (SubState::Inactive, SubState::Active(entered)) => {
            world.try_run_schedule(OnEnter(entered)).ok();
        }
        (SubState::Inactive, SubState::Inactive) => {}
    }
}

pub trait InitHierarchicalState {
    fn add_root_state<S: States>(&mut self);
    fn add_substate<S: States, P: States>(&mut self, parent: P);
}

impl InitHierarchicalState for App {
    fn add_root_state<S: States>(&mut self) {
        // Root states start as inactive
        self.insert_resource(State::new(SubState::<S>::Inactive));
        self.init_resource::<NextState<SubState<S>>>();
        self.add_systems(
            StateTransition,
            (
                // This sets the default state for root states
                run_enter_schedule::<S>.run_if(run_once()),
                apply_on_transition::<S>,
            )
                .chain(),
        );
    }

    fn add_substate<S: States, P: States>(&mut self, parent: P) {
        // Substates are set by parents during their OnEnter schedule
        self.init_resource::<State<SubState<S>>>();
        self.init_resource::<NextState<SubState<S>>>();
        self.add_systems(
            OnEnter(parent.clone()),
            change_state(SubState::Active(S::default())),
        );
        self.add_systems(OnExit(parent), change_state(SubState::<S>::Inactive));
        self.add_systems(
            StateTransition,
            (apply_on_transition::<S>.after(apply_on_transition::<P>),).chain(),
        );
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
    pub enum MajorState {
        #[default]
        Major1,
        Major2,
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
    pub enum MinorState {
        #[default]
        Minor1,
    }

    use crate::{change_state, InitHierarchicalState, SubState};

    #[test]
    fn major_1_minor_1() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_root_state::<MajorState>();
        app.add_substate::<MinorState, MajorState>(MajorState::Major1);

        app.update();

        assert_eq!(
            *app.world.resource::<State<SubState<MajorState>>>().get(),
            SubState::Active(MajorState::Major1)
        );
        assert_eq!(
            *app.world.resource::<State<SubState<MinorState>>>().get(),
            SubState::Active(MinorState::Minor1)
        );
    }

    #[test]
    fn major_2_minor_inactive() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_root_state::<MajorState>();
        app.add_substate::<MinorState, MajorState>(MajorState::Major1);
        app.add_systems(
            Startup,
            change_state(SubState::Active(MajorState::Major2)).run_if(run_once()),
        );

        app.update();

        assert_eq!(
            *app.world.resource::<State<SubState<MajorState>>>().get(),
            SubState::Active(MajorState::Major2)
        );
        assert_eq!(
            *app.world.resource::<State<SubState<MinorState>>>().get(),
            SubState::Inactive
        );
    }
}
