use bevy::{
    math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide, time::FixedTimestep, transform,
};

// using strg . here in vsc to gain knowledge about imports
use crate::{
    components::{Enemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity},
    GameTextures, PlayerState, WinSize, BASE_SPEED, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY,
    PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn_system),
            )
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

// - add player
fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    //prep playerState
    let now = time.seconds_since_startup();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        //add player
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

        player_state.spawned();
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    //this is a good place to keep player inside the windo w

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
