use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
    Menu,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    health: u32,
}

#[derive(Resource, Default)]
struct Game {
    player: Player,
    score: u32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        // .init_state::<GameState>()
        .add_systems(Startup, (setup_cameras, setup))
        // .add_systems(
        //     Update,
        //     (
        //         move_player,
        //         focus_camera,
        //         rotate_bonus,
        //         scoreboard_system,
        //         spawn_bonus,
        //     ), // .run_if(in_state(GameState::Playing)),
        // )
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    game.score = 0;

    //player is a red circle in the middle of the screen
    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: Vec2::new(0., 0.).extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
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

    commands.spawn(
        TextBundle::from_section(
            "Texto:",
            TextStyle {
                font_size: 40.0,
                color: Color::srgb(0.5, 0.5, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}
