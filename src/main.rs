use bevy::prelude::*;

use CastleVaniaGame::{
    control::ControlPlugin,
    player::PlayerPlugin,
    map::MapPlugin,
    assets::AssetPlugin,
    level::LevelPlugin,
    game_over::GameOverPlugin,
    background_generator::BackgroundGeneratorPlugin,

    player::CameraFollow,
    map::{PLAYER_START_X, PLAYER_START_Y},
};

use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::render::camera::ClearColorConfig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Castlevania Fan Game".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            AssetPlugin, 
            ControlPlugin, 
            PlayerPlugin, 
            MapPlugin, 
            BackgroundGeneratorPlugin,
            LevelPlugin,
            GameOverPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    
    camera.camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.6));
    camera.projection.scale = 0.05;
    camera.transform.translation = Vec3::new(
        PLAYER_START_X as f32,
        PLAYER_START_Y as f32,
        camera.transform.translation.z,
    );
    
    commands.spawn((camera, CameraFollow));
}