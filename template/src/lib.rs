use bevy::prelude::*;

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, template_pattern);
    }
}

#[derive(Default, Resource)]
pub struct TemplatePattern;

fn template_pattern(mut commands: Commands) {
    commands.init_resource::<TemplatePattern>();
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{TemplatePattern, TemplatePlugin};

    #[test]
    fn template_test() {
        // Arrange
        // Some patterns may require visual elements
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(TemplatePlugin);

        // Act
        // Run multiple ticks if necessary
        app.update();

        // Assert
        // Check all assumptions and nothing else
        assert!(app.world.contains_resource::<TemplatePattern>());
    }
}
