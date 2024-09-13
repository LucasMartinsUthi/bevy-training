use crate::{health_bar::HealthBar, DamageTimer, Enemy};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Default, Component)]
pub struct Weapon {
    pub damage: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct WeaponSprite;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn);
        app.add_systems(Update, (orbit, weapon_hit_enemy));
    }
}

fn spawn(mut commands: Commands, query: Query<(Entity, &Weapon), With<Weapon>>) {
    for (entity, _weapon) in query.iter() {
        let children = commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 120., 1.),
                    scale: Vec3::new(8., 120., 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb(0.1, 0.1, 0.1),
                    ..default()
                },
                ..default()
            })
            .insert(WeaponSprite)
            .id();

        commands.entity(entity).push_children(&[children]);
    }
}

fn orbit(
    mut query: Query<(&Parent, &mut Transform), With<WeaponSprite>>,
    weapon_query: Query<&Weapon>,
    time: Res<Time>,
) {
    for (parent, mut transform) in query.iter_mut() {
        let entity = parent.get();

        if let Ok(weapon) = weapon_query.get(entity) {
            //sprite must orbit around the parent entity
            let rotation = Quat::from_rotation_z(time.delta_seconds() * weapon.rotation_speed);
            transform.translation = rotation.mul_vec3(transform.translation);
            transform.rotate(rotation);
        }
    }
}

pub fn weapon_hit_enemy(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform, &mut HealthBar, &mut DamageTimer), With<Enemy>>,
    weapon_query: Query<(&Weapon, &Transform), With<Weapon>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    for (enemy, enemy_transform, mut health_bar, mut damage_timer) in enemy_query.iter_mut() {
        for (weapon, weapon_transform) in weapon_query.iter() {
            let distance = weapon_transform
                .translation
                .distance(enemy_transform.translation);

            damage_timer.0.tick(time.delta());

            if damage_timer.0.elapsed_secs() >= 1.0 && distance < 60.0 {
                println!("Distance: {}", distance);
                health_bar.health -= weapon.damage;

                damage_timer.0.reset();

                println!("Enemy health: {}", health_bar.health);

                if health_bar.health <= 0.0 {
                    commands.entity(enemy).despawn_recursive();

                    audio.play(asset_server.load("audio/explosionCrunch_000.ogg"));
                }
            }
        }
    }
}
