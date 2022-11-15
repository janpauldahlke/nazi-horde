#![allow(unused)] // silence warnings while dev // comment out later

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::entity,
    math::Vec3Swizzles,
    prelude::*,
    sprite::collide_aabb::collide,
    utils::HashSet,
};
use components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromPlayer, Laser, Movable, Player,
    SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
mod player;

// region: --- Asset constants
const PLAYER_SPRITE: &str = "black_jesus.png";
const PLAYER_SIZE: (f32, f32) = (144., 177.);
const PLAYER_LASER_SPRITE: &str = "player_laser.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

const ENEMY_SPRITE: &str = "enemy_b.png";
const ENEMY_SIZE: (f32, f32) = (67., 144.);
const ENEMY_LASER_SPRITE: &str = "enemy_laser.png";
const ENEMY_LASER_SIZE: (f32, f32) = (9., 54.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

// endregion: --- Asset constants

// region: --- Game constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
const ENEMY_MAX: u32 = 100;
// endregion: --- Game constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

pub struct EnemyCount(u32);
pub struct KillCount(u32);
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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(player_laser_hit_enemy_system)
        .add_system(explosion_to_spawn_system)
        .add_system(explosion_animation_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
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

    //create explosion texture
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlasses.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
    commands.insert_resource(KillCount(0));
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
            const MARGIN: f32 = 230.;

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

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut kill_count: ResMut<KillCount>,
    laser_query: Query<(
        Entity,
        &Transform,
        &SpriteSize,
        (With<Laser>, With<FromPlayer>),
    )>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    // helper avoids despawning multiple times
    // cross check against another set to avoid double destroy on no longer existing entity
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iteratre through lasers
    for (laser_entity, laser_tf, laser_size, _) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        //let laser_scale = Vec2::from(laser_tf.scale.xy());
        let laser_scale: Vec2 = Vec2::from(laser_tf.scale.xy());
        // iterate through enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // introducing collided
            // --collision logic
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision, if collision
            if let Some(_) = collision {
                //remove enemy entity using despawn
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;
                kill_count.0 += 1;
                //remove laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);
                //spwan explosion
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_tf.translation.clone()));
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn the explosionToDespawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // how to move to next on sprite (textureatlas) index
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn();
            }
        }
    }
}
