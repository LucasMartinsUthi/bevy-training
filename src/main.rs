use bevy::prelude::*;
use bevy_training::player::PlayerPlugin;
use bevy_training::Game;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .init_resource::<Game>()
        // .init_state::<GameState>()
        .add_systems(Startup, (setup_cameras, setup))
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    game.score = 0;

    commands.spawn(
        TextBundle::from_section(
            "Texto: 1",
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
