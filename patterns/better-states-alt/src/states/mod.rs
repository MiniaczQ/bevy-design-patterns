mod app;
mod gameplay;
mod running;

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{
        states::{
            app::{AppState, SsfAppState},
            gameplay::{GameplayState, SsfGameplayState},
            running::{RunningState, SsfRunningState},
        },
        AddStateV2, StateComparator,
    };

    fn change_state<S: States + StateComparator>(state: S) -> impl Fn(ResMut<NextState<S>>) {
        move |mut next: ResMut<NextState<S>>| next.set(state.clone())
    }

    #[test]
    fn default_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_state_v2::<AppState>();

        app.update();

        let state = app.world.resource::<State<AppState>>();
        dbg!(state);
    }

    #[test]
    fn changed_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_state_v2::<AppState>();
        app.add_systems(
            Startup,
            change_state(AppState::Gameplay(GameplayState {
                running: RunningState::Running,
                difficulty: 0,
            })),
        );

        app.update();

        let state = app.world.resource::<State<AppState>>();
        dbg!(state);
    }

    #[test]
    fn mainmenu_to_running() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_state_v2::<AppState>();
        app.add_systems(
            Startup,
            change_state(AppState::Gameplay(GameplayState {
                running: RunningState::Running,
                // It'd be nice to skip this, but we need to specify it
                difficulty: 0,
            })),
        );
        app.add_systems(OnExit(SsfAppState::MainMenu), || println!("1"));
        app.add_systems(
            OnTransition {
                from: SsfAppState::MainMenu,
                to: SsfAppState::Gameplay,
            },
            || println!("2"),
        );
        app.add_systems(OnEnter(SsfAppState::Gameplay), || println!("3"));
        app.add_systems(
            OnEnter(SsfGameplayState {
                // It'd be nice to pattern match here, but we need to specify it
                difficulty: 0,
            }),
            || println!("4"),
        );
        app.add_systems(OnEnter(SsfRunningState::Running), || println!("5"));

        app.update();

        let state = app.world.resource::<State<AppState>>();
        dbg!(state);
    }
}
