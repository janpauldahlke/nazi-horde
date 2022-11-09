#![allow(unused)] // silence warnings while dev // comment out later

use bevy::prelude::*;

// region: --- Asset constants
const PLAYER_SPRITE: &str = "black_jesus.png";
const PLAYER_SIZE: (f32, f32) = (144., 177.);
const SPRITE_SCALE: f32 = 0.5;
const ENEMY_SPRITE: &str = "enemy_a_01.png";
// endregion: --- Asset constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
// endregion: --- Resource

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Nazi Horde".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // - Camera
    commands.spawn_bundle(Camera2dBundle::default()); //https://bevyengine.org/learn/book/migration-guides/0.7-0.8/

    //capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    //position window
    window.set_position(IVec2::new(2780, 4900));
    // size window
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}

// - add player
fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
