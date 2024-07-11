use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_systems(Startup, (setup, test).chain());

    app.run();
}

/// Logic that uses the pattern.
fn setup(mut commands: Commands) {
    commands.spawn((Number(5), NumberPlusOne(0)));
}

fn test(query: Query<&NumberPlusOne>) {
    println!("Number plus one: {}", query.single().0);
}

/// Pattern core logic.W
#[derive(Component)]
struct Number(u32);

struct NumberPlusOne(u32);

impl Component for NumberPlusOne {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let number = world
                .get::<Number>(entity)
                .expect("Cannot add `NeedsNumber`, because `Numer` is missing.")
                .0;
            world.get_mut::<NumberPlusOne>(entity).unwrap().0 = number + 1;
        });
    }
}
