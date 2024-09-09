use bevy::prelude::*;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // app.add_systems(Update, (follow,));
    }
}

#[derive(Default, Component)]
pub struct HealthBar {
    max_health: u32,
    health: u32,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec2::new(0., 0.).extend(0.0),
                scale: Vec2::new(50., 5.).extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(1., 1., 1.),
                ..default()
            },
            ..default()
        },
        HealthBar {
            max_health: 100,
            health: 100,
        },
    ));
}
