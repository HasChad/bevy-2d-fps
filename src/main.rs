use bevy::{prelude::*, window::WindowMode};
use ingame::InGamePlugin;
use mainmenu::MainMenuPlugin;

pub mod ingame;
pub mod mainmenu;
pub mod gameover;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy 2D FPS".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    resizable: false,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )
        .add_systems(Startup, setup)
        .add_state::<AppState>()
        .add_plugins(InGamePlugin)
        .add_plugins(MainMenuPlugin)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //setup camera with debug-render.
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SpriteBundle{
            texture: asset_server.load("sprites\\background.png"),
            transform: Transform::from_xyz(0.0, 0.0, -5.0),
            ..default()
    });
}