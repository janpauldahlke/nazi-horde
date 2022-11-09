#![allow(unused)] // silence warnings while dev // comment out later

use bevy::prelude::*;

// region: -- Asset constants
// --player
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

// --enemy
const ENEMY_SPRITE: &str = "enemy_a_01.png";
// endregion: -- Asset constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Nazi Horde".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // - Camera
    commands.spawn_bundle(Camera2dBundle::default()); //https://bevyengine.org/learn/book/migration-guides/0.7-0.8/
                                                      // -Rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
