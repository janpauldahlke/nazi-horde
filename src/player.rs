use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, transform};

// using strg . here in vsc to gain knowledge about imports
use crate::{
    components::{Enemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    GameTextures, WinSize, BASE_SPEED, PLAYER_LASER_SIZE, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

// - add player
fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 1.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0., y: 0. });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

// --- jeremy says "that is a magic"
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    };
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            //let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 15.;
            let y_offset = PLAYER_SIZE.1 / 2. * SPRITE_SCALE;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + y_offset - 2., 2.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Movable { auto_despawn: true })
                    .insert(Velocity { x: 0., y: 1. });
            };

            spawn_laser(PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 47.);
            spawn_laser(PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 37.);
        }
    }
}
