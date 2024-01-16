use bevy::prelude::*;

pub struct ChildEntityInstallerPlugin;

impl Plugin for ChildEntityInstallerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, child_entity_installer);
    }
}

#[derive(Component)]
struct InstallHere;

fn child_entity_installer(mut commands: Commands, query: Query<Entity, Added<InstallHere>>) {
    for entity_id in query.iter() {
        commands.spawn(()).set_parent(entity_id);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{ChildEntityInstallerPlugin, InstallHere};

    #[test]
    fn install_from_spawn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(ChildEntityInstallerPlugin);
        app.world.spawn(InstallHere);

        app.update();

        assert!(
            app.world
                .query_filtered::<&Children, With<InstallHere>>()
                .single(&app.world)
                .len()
                == 1
        );
    }

    #[test]
    fn install_after_spawn() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(ChildEntityInstallerPlugin);
        let id = app.world.spawn(()).id();
        app.add_systems(
            Update,
            (move |mut commands: Commands| {
                commands.entity(id).insert(InstallHere);
            })
            .run_if(run_once()),
        );

        app.update();
        app.update();

        assert!(
            app.world
                .query_filtered::<&Children, With<InstallHere>>()
                .single(&app.world)
                .len()
                == 1
        );
    }
}
