use bevy::{ecs::query, prelude::*};
use std::marker::PhantomData;

use crate::health_bar;

#[derive(Component)]
pub struct HealthBarPlugin<T: Component> {
    _marker: PhantomData<T>, // Using PhantomData to make T a valid part of the struct
    max_health: u32,
    health: u32,
    entity: Option<Entity>,
}

impl<T: Component> Default for HealthBarPlugin<T> {
    fn default() -> Self {
        HealthBarPlugin {
            _marker: PhantomData,
            max_health: 100,
            health: 100,
            entity: None,
        }
    }
}

impl<T: Component> Plugin for HealthBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::<T>);
    }
}

fn setup<T: Component>(mut commands: Commands, mut query: Query<&mut HealthBarPlugin<T>>) {
    println!("Setting up Player");

    match query.get_single_mut() {
        Ok(mut health_bar) => {
            health_bar.entity = Some(
                commands
                    .spawn(SpriteBundle {
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
                    })
                    .id(),
            );
        }
        Err(err) => {
            println!("Error querying HealthBarPlugin<T>: {:?}", err);
        }
    };
}
