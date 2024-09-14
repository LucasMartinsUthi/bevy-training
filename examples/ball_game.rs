use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_training::{
    health_bar::{HealthBar, HealthBarPlugin},
    weapon::{Weapon, WeaponPlugin},
    DamageTimer, Enemy, Player,
};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub const PLAYER_ENEMY: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(EditorPlugin::default())
        .add_plugins(HealthBarPlugin)
        .add_plugins(WeaponPlugin)
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemy))
        .add_systems(
            Update,
            (
                player_movement,
                bound_player_movement,
                enemy_movement,
                bound_enemy_movement,
                enemy_hit_player,
            ),
        )
        .add_systems(PostUpdate, despawn_enemys)
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            )),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..Default::default()
        })
        .insert(Player)
        .insert(HealthBar {
            max_health: 200.,
            health: 200.,
        })
        .insert(DamageTimer::default())
        .insert(Weapon {
            damage: 10.0,
            rotation_speed: 5.0,
            ..Default::default()
        });
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        )),
        ..Default::default()
    });
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..PLAYER_ENEMY {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands
            .spawn(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 0.0)),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..Default::default()
            })
            .insert(Enemy {
                direction: Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5).normalize(),
            })
            .insert(HealthBar {
                max_health: 50.,
                health: 50.,
            })
            .insert(DamageTimer::default())
            .insert(Collider::ball(32.))
            .insert(RigidBody::Fixed)
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec3::Y;
        }

        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::X;
        }

        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::X;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn bound_player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_size = PLAYER_SIZE / 2.0;
        let x_min = half_size;
        let x_max = window.width() - half_size;
        let y_min = half_size;
        let y_max = window.height() - half_size;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}

pub fn enemy_movement(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn bound_enemy_movement(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (mut transform, mut enemy) in query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let half_size = ENEMY_SIZE / 2.0;
        let x_min = half_size;
        let x_max = window.width() - half_size;
        let y_min = half_size;
        let y_max = window.height() - half_size;
        let mut direction_changed = false;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }

        if transform.translation.y <= y_min || transform.translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if transform.translation.x <= x_min || transform.translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_handle = if random::<f32>() > 0.5 {
                asset_server.load("audio/pluck_001.ogg")
            } else {
                asset_server.load("audio/pluck_002.ogg")
            };

            // Play the audio directly without spawning an entity
            audio.play(sound_handle);
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut HealthBar, &mut DamageTimer), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_transform, mut health_bar, mut damage_timer)) =
        player_query.get_single_mut()
    {
        for enemy in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy.translation);

            damage_timer.0.tick(time.delta());

            if damage_timer.0.elapsed_secs() >= 1.0
                && distance < PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0
            {
                health_bar.health -= 10.0;

                damage_timer.0.reset();

                println!("Player health: {}", health_bar.health);

                if health_bar.health <= 0.0 {
                    commands.entity(player_entity).despawn_recursive();
                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/explosionCrunch_000.ogg"),
                        ..default()
                    });
                }
            }
        }
    }
}

fn despawn_enemys(
    mut commands: Commands,
    query: Query<(Entity, &HealthBar), With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (entity, health_bar) in query.iter() {
        if health_bar.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
            audio.play(asset_server.load("audio/explosionCrunch_000.ogg"));
        }
    }
}
