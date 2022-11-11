#![allow(unused)] // silence warnings while dev // comment out later

use bevy::prelude::*;
use components::{Movable, Player, Velocity};
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Asset constants
const PLAYER_SPRITE: &str = "black_jesus.png";
const PLAYER_SIZE: (f32, f32) = (144., 177.);
const PLAYER_LASER_SPRITE: &str = "player_laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

//const ENEMY_SPRITE: &str = "enemy_a_01.png";
// endregion: --- Asset constants

// region: --- Game constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
// endregion: --- Game constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}
// endregion: --- Resource

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "<---- BJ vs. infinite Hitlers ---->".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
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

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if (movable.auto_despawn) {
            // --- despwan lasers out of screen
            const MARGIN: f32 = 200.;

            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.h / 2. + MARGIN
                || translation.x < -win_size.h / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
