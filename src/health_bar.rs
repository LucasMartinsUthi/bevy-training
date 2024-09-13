use bevy::{asset::transformer, prelude::*};

use crate::health_bar;

#[derive(Default, Component)]
pub struct HealthBar {
    pub max_health: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct HealthBarSprite;

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn);
        app.add_systems(Update, update_health_bar);
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
            .insert(HealthBarSprite)
            .id();

        commands.entity(entity).push_children(&[children]);
    }
}

fn update_health_bar(
    mut commands: Commands,
    mut query: Query<(&Parent, &mut Transform), With<HealthBarSprite>>,
    health_bar_query: Query<&HealthBar>,
) {
    for (parent, mut transform) in query.iter_mut() {
        let entity = parent.get();

        if let Ok(health_bar) = health_bar_query.get(entity) {
            transform.scale = Vec3::new(health_bar.health, 5., 1.);
        }
    }
}
