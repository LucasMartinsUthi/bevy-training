use bevy::{asset::transformer, prelude::*};

use crate::health_bar;

#[derive(Default, Component)]
pub struct HealthBar {
    pub max_health: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct HealthBarFlag;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn, update_health_bar));
    }
}

fn spawn(mut commands: Commands, query: Query<(Entity, &HealthBar), With<HealthBar>>) {
    for (entity, health_bar) in query.iter() {
        let children = commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 20., 1.),
                    scale: Vec3::new(health_bar.health, 5., 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb(1., 0.2, 0.2),
                    ..default()
                },
                ..default()
            })
            .insert(HealthBarFlag)
            .id();

        commands.entity(entity).push_children(&[children]);
        // println!("Spawned health bar for entity {:?}", entity);
    }
}

fn update_health_bar(
    health_bar_query: Query<(Entity, &HealthBar), With<HealthBar>>,
    mut transform_query: Query<&mut Transform, With<HealthBarFlag>>,
) {
    // println!("Updating health bar");

    for (_entity, health_bar) in health_bar_query.iter() {
        for mut transform in transform_query.iter_mut() {
            transform.scale.x = health_bar.health;
        }
    }
}
