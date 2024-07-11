use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(plugin);

    app.add_systems(Update, test);

    app.run();
}

/// Logic that uses the pattern.
fn test(resource: Option<Res<TemplatePattern>>) {
    info!("{:?}", resource);
}

/// Pattern core logic.
fn plugin(app: &mut App) {
    app.add_systems(Startup, template_pattern);
}

#[derive(Resource, Default, Debug)]
pub struct TemplatePattern;

fn template_pattern(mut commands: Commands) {
    commands.init_resource::<TemplatePattern>();
}
