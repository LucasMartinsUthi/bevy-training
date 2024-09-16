use crate::{health_bar::HealthBar, DamageTimer, Enemy};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WEAPON_TICK_SECS: f32 = 0.5;

#[derive(Resource)]
pub struct WeaponTick {
    pub timer: Timer,
}

impl Default for WeaponTick {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(WEAPON_TICK_SECS, TimerMode::Once),
        }
    }
}

#[derive(Default)]
pub enum RotationDirection {
    #[default]
    Clockwise,
    CounterClockwise,
}

#[derive(Default, Component)]
struct WeaponDebug;

#[derive(Default, Component)]
pub struct Weapon {
    pub damage: f32,
    pub rotation_speed: f32,
    pub current_rotation: f32,
    pub rotation_direction: RotationDirection,
}

#[derive(Component)]
pub struct WeaponSprite;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponTick>();
        app.add_systems(PostStartup, spawn);
        app.add_systems(PreUpdate, change_direction);
        app.add_systems(Update, (orbit, display_events, weapon_hit_enemy));
        // app.add_systems(PostUpdate, (debug_weapon));
        // app.add_systems(PreUpdate, (despawn_debug));
    }
}

fn spawn(mut commands: Commands, query: Query<(Entity, &Weapon), With<Weapon>>) {
    for (entity, _weapon) in query.iter() {
        let children = commands
            .spawn(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 120., 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(8.0, 120.0)),
                    ..default()
                },
                ..default()
            })
            .insert(WeaponSprite)
            .insert(Collider::cuboid(4., 60.))
            .insert(Sensor)
            .id();

        commands.entity(entity).push_children(&[children]);
    }
}

fn orbit(
    mut query: Query<(&Parent, &mut Transform), With<WeaponSprite>>,
    mut weapon_query: Query<&mut Weapon>,
    time: Res<Time>,
) {
    for (parent, mut transform) in query.iter_mut() {
        let entity = parent.get();

        if let Ok(mut weapon) = weapon_query.get_mut(entity) {
            let speed = match weapon.rotation_direction {
                RotationDirection::Clockwise => weapon.rotation_speed,
                RotationDirection::CounterClockwise => -weapon.rotation_speed,
            };

            let rotation = Quat::from_rotation_z(time.delta_seconds() * speed);
            transform.translation = rotation.mul_vec3(transform.translation);
            transform.rotate(rotation);

            let rotation = transform.rotation.to_axis_angle();
            weapon.current_rotation = rotation.1;
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

pub fn weapon_hit_enemy(
    rapier_context: Res<RapierContext>,
    q_weapon_sprite: Query<(Entity, &Collider, &Transform, &GlobalTransform), With<WeaponSprite>>,
    q_damage: Query<&Weapon>,
    mut q_enemies: Query<&mut HealthBar, With<Enemy>>,
    time: Res<Time>,
    mut tick: ResMut<WeaponTick>,
) {
    let damage = q_damage.get_single().unwrap().damage;
    let (weapon, collider, transform, g_transform) = q_weapon_sprite.get_single().unwrap();

    let filter = QueryFilter {
        exclude_collider: Some(weapon),
        ..default()
    };

    tick.timer.tick(time.delta());

    rapier_context.intersections_with_shape(
        g_transform.translation().truncate(),
        transform.rotation.to_euler(EulerRot::ZYX).0,
        collider,
        filter,
        |entity| {
            if let Ok(mut health_bar) = q_enemies.get_mut(entity) {
                if tick.timer.finished() {
                    health_bar.health -= damage;
                    tick.timer.reset();
                }
            }
            true
        },
    );
}

fn change_direction(mut query: Query<&mut Weapon>, input: Res<ButtonInput<KeyCode>>) {
    let mut weapon = query.get_single_mut().unwrap();

    if input.just_pressed(KeyCode::Space) {
        weapon.rotation_direction = match weapon.rotation_direction {
            RotationDirection::Clockwise => RotationDirection::CounterClockwise,
            RotationDirection::CounterClockwise => RotationDirection::Clockwise,
        };
    }
}
