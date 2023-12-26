use bevy::{ecs::system::RunSystemOnce, prelude::*};

/// State activity status.
/// All states are stored in `StateActivity` starting as `Inactive`.
/// Root states are set to `Active` during `Startup` schedule and never go `Inactive` again.
/// Substates are set to `Active` and `Inactive` during their parent's respective `OnEnter` and `OnExit` schedules.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum StateActivity<S: States> {
    #[default]
    Inactive,
    Active(S),
}

impl<S: States> StateActivity<S> {
    pub fn is_active(&self) -> bool {
        match self {
            StateActivity::Inactive => false,
            StateActivity::Active(_) => true,
        }
    }
}

/// Modified `run_enter_schedule`.
/// Sets root states to active with their `NextState` or default value.
/// This system is no longer responsible for running the `OnEnter` schedule.
pub fn run_enter_schedule<S: States>(world: &mut World) {
    let next_state = world.resource_mut::<NextState<S>>().0.take();
    let mut internal_next_state = world.resource_mut::<NextState<StateActivity<S>>>();
    match next_state {
        Some(state) => internal_next_state.set(StateActivity::Active(state)),
        None => internal_next_state.set(StateActivity::Active(S::default())),
    }
}

/// Helper function for changing state.
pub fn change_state<S: States>(state: S) -> impl Fn(ResMut<NextState<S>>) {
    move |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    }
}

pub fn propagate_states<S: States>(
    mut internal_next_state: ResMut<NextState<StateActivity<S>>>,
    mut next_state: ResMut<NextState<S>>,
    mut state: ResMut<State<StateActivity<S>>>,
) -> Option<(StateActivity<S>, StateActivity<S>)> {
    let mut internal_next_state = internal_next_state.bypass_change_detection().0.take();

    if state.is_active() || internal_next_state.clone().is_some_and(|x| x.is_active()) {
        if let Some(next_state) = next_state.0.take() {
            internal_next_state.replace(StateActivity::Active(next_state));
        }
    }

    let Some(entered) = internal_next_state else {
        return None;
    };

    if *state == entered {
        return None;
    }

    // TODO: use `mem::replace` when integrating with Bevy cause private fields
    let exited = state.get().clone();
    *state = State::new(entered.clone());

    Some((exited, entered))
}

/// Heavily modified `apply_on_transition`.
/// Only runs transition schedules if all involved states are active.
/// - OnExit - if exited an active state
/// - OnTransition - if exited and entered active states
/// - OnEnter - if entered an active state
pub fn apply_on_transition<S: States>(world: &mut World) {
    let Some((exited, entered)) = world.run_system_once(propagate_states::<S>) else {
        return;
    };

    match (exited, entered) {
        (StateActivity::Active(exited), StateActivity::Inactive) => {
            world.try_run_schedule(OnExit(exited)).ok();
        }
        (StateActivity::Active(exited), StateActivity::Active(entered)) => {
            world.try_run_schedule(OnExit(exited.clone())).ok();
            world
                .try_run_schedule(OnTransition {
                    from: exited,
                    to: entered.clone(),
                })
                .ok();
            world.try_run_schedule(OnEnter(entered)).ok();
        }
        (StateActivity::Inactive, StateActivity::Active(entered)) => {
            world.try_run_schedule(OnEnter(entered)).ok();
        }
        (StateActivity::Inactive, StateActivity::Inactive) => {}
    }
}

pub trait AppSubstateExt {
    /// Adds a root state to the app.
    /// A root state is always defined.
    fn add_root_state<S: States>(&mut self);

    /// Adds a substate to the app for a given variant of parent state.
    /// A substate is only defined if the parent state is the correct variant.
    ///
    /// States create a tree, not a graph, hence can be added only once as a substate.
    /// Adding it multiple times can cause system ordering issues.
    fn add_substate<P: States, S: States>(&mut self, parent: P);
}

impl AppSubstateExt for App {
    fn add_root_state<S: States>(&mut self) {
        add_common_resources::<S>(self);
        self.add_systems(
            StateTransition,
            (
                run_enter_schedule::<S>.run_if(run_once()),
                apply_on_transition::<S>,
            )
                .chain(),
        );
    }
    fn add_substate<P: States, S: States>(&mut self, parent: P) {
        add_common_resources::<S>(self);
        self.add_systems(OnEnter(parent.clone()), set_active_next_or_default::<S>);
        self.add_systems(OnExit(parent), set_inactive::<S>);
        self.add_systems(
            StateTransition,
            apply_on_transition::<S>.after(apply_on_transition::<P>),
        );
    }
}

/// Adds common resources for root states and substates.
fn add_common_resources<S: States>(app: &mut App) {
    // The state itself.
    app.init_resource::<State<StateActivity<S>>>();
    // Internal `NextState`, contains information about activity.
    // Could possibly introduce a different struct.
    app.init_resource::<NextState<StateActivity<S>>>();
    // User-facing `NextState`, only used when selected state is `Active`.
    app.init_resource::<NextState<S>>();
}

/// Helper function for changing state.
pub fn set_active_next_or_default<S: States>(
    mut next_state: ResMut<NextState<S>>,
    mut internal_next_state: ResMut<NextState<StateActivity<S>>>,
) {
    let state = next_state.0.take().unwrap_or_default();
    internal_next_state.set(StateActivity::Active(state));
}

/// Helper function for changing state.
pub fn set_inactive<S: States>(
    mut next_state: ResMut<NextState<S>>,
    mut internal_next_state: ResMut<NextState<StateActivity<S>>>,
) {
    next_state.0.take();
    internal_next_state.set(StateActivity::Inactive);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{AppSubstateExt, StateActivity};

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
    pub enum AppState {
        #[default]
        MainMenu,
        Gameplay,
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
    pub enum GameplayState {
        #[default]
        Playing,
        Paused,
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_root_state::<AppState>();
        app.add_substate::<AppState, GameplayState>(AppState::Gameplay);
        app
    }

    #[test]
    fn inactive() {
        let app = setup();

        // No update

        assert_inactive::<AppState>(&app);
        assert_inactive::<GameplayState>(&app);
    }

    #[test]
    fn mainmenu() {
        let mut app = setup();
        // No initialization, `MainMenu` is default for `AppState`.

        app.update();

        assert_active(&app, AppState::MainMenu);
        assert_inactive::<GameplayState>(&app);
    }

    #[test]
    fn playing() {
        let mut app = setup();
        // Initialize `AppState` with `MainMenu`, `Playing` is `GameplayState` default.
        app.insert_resource(NextState(Some(AppState::Gameplay)));

        app.update();

        assert_active(&app, AppState::Gameplay);
        assert_active(&app, GameplayState::Playing);
    }

    #[test]
    fn paused() {
        let mut app = setup();
        // Initialize `AppState` with `MainMenu` and `GameplayState` with `Paused`.
        app.insert_resource(NextState(Some(AppState::Gameplay)));
        app.insert_resource(NextState(Some(GameplayState::Paused)));

        app.update();

        assert_active(&app, AppState::Gameplay);
        assert_active(&app, GameplayState::Paused);
    }

    #[test]
    fn mainmenu_to_playing() {
        let mut app = setup();
        app.update();

        assert_active(&app, AppState::MainMenu);
        assert_inactive::<GameplayState>(&app);

        app.insert_resource(NextState(Some(AppState::Gameplay)));
        app.update();

        assert_active(&app, AppState::Gameplay);
        assert_active(&app, GameplayState::Playing);
    }

    #[test]
    fn mainmenu_to_paused() {
        let mut app = setup();
        app.update();

        assert_active(&app, AppState::MainMenu);
        assert_inactive::<GameplayState>(&app);

        app.insert_resource(NextState(Some(AppState::Gameplay)));
        app.insert_resource(NextState(Some(GameplayState::Paused)));
        app.update();

        assert_active(&app, AppState::Gameplay);
        assert_active(&app, GameplayState::Paused);
    }

    #[test]
    fn paused_to_mainmenu() {
        let mut app = setup();
        app.insert_resource(NextState(Some(AppState::Gameplay)));
        app.insert_resource(NextState(Some(GameplayState::Paused)));
        app.update();

        assert_active(&app, AppState::Gameplay);
        assert_active(&app, GameplayState::Paused);

        app.insert_resource(NextState(Some(AppState::MainMenu)));
        app.update();

        assert_active(&app, AppState::MainMenu);
        assert_inactive::<GameplayState>(&app);
    }

    fn assert_active<S: States>(app: &App, state: S) {
        assert_eq!(
            *app.world.resource::<State<StateActivity<S>>>().get(),
            StateActivity::Active(state)
        );
    }

    fn assert_inactive<S: States>(app: &App) {
        assert_eq!(
            *app.world.resource::<State<StateActivity<S>>>().get(),
            StateActivity::Inactive
        );
    }
}
