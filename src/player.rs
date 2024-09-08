use crate::health_bar::HealthBarPlugin;
use crate::Game;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (movement,));
        app.add_plugins(HealthBarPlugin::<Player>::default());
    }
}

#[derive(Default, Component)]
pub struct Player {
    pub entity: Option<Entity>,
    // pub health_bar: HealthBar,
    pub health: u32,
    pub x: f32,
    pub y: f32,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    println!("Setting up Player");

    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec2::new(game.player.x, game.player.y).extend(0.0),
                    scale: Vec2::new(10., 10.).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb(1., 0.2, 0.2),
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    // game.player.health_bar.setup(&mut commands);
}

fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        game.player.y += 10.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        game.player.y -= 10.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        game.player.x += 10.;
        moved = true;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        game.player.x -= 10.;
        moved = true;
    }

    if moved {
        if let Some(entity) = game.player.entity {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.translation = Vec3::new(game.player.x, game.player.y, 0.0);
            }
        }
    }
}
