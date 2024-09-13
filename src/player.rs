use crate::health_bar::HealthBar;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (movement,));
    }
}

#[derive(Default, Component)]
pub struct Player {
    pub health: u32,
    pub x: f32,
    pub y: f32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(10., 10., 1.),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(1., 0.2, 0.2),
                ..default()
            },
            ..default()
        })
        .insert(Player {
            health: 100,
            x: 0.,
            y: 0.,
        })
        .insert(HealthBar {
            max_health: 100.,
            health: 100.,
        });
}

fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut();
    let mut transform = transform_query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        player.y += 10.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        player.y -= 10.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        player.x += 10.;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        player.x -= 10.;
    }

    transform.translation = Vec3::new(player.x, player.y, 0.0);
}
