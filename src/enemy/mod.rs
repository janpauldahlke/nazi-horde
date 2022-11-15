use std::f32::consts::PI;

use crate::{
    components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity},
    EnemyCount, GameTextures, WinSize, BASE_SPEED, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE,
    SPRITE_SCALE, TIME_STEP,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*, time::FixedTimestep, transform};
use rand::{thread_rng, Rng};

use self::formation::{Formation, FormationMaker};

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        //add enemy a little bit delay to stage
        app.insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.2))
                    .with_system(enemy_spawn_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system),
            )
            .add_system(enemy_move_system);

        //app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
        //app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        /// get formation and start x/y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

//TODO: enemy out of sight is not despawning
// all enemies share the same movement pattern

fn enemy_move_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>,
) {
    let now = time.seconds_since_startup() as f32; // casting;
    for (mut transform, mut formation) in query.iter_mut() {
        //current position
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);

        //max distance
        let max_distance = TIME_STEP * formation.speed;

        //fixtures, hardcoded for now

        let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. }; //  1 counter clockwise,  -1 clockwise
        let (x_pivot, y_pivot) = formation.pivot;
        let (x_radius, y_radius) = formation.radius; //kind of ellipse shape

        //compute next angle (based on time for now)
        let angle = formation.angle
            + dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

        // compute target x/y
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.cos() + y_pivot;

        // compute distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        // compute final x/y
        let x = x_org - dx * distance_ratio;
        //shallow
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        //shallow again
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // start rotating the formation angle only when sprite iis on or close to ellipse
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    // 60. is kinda magic number for framerate equivalent
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        //spawn enemy laser
        let (x, y) = (tf.translation.x, tf.translation.y);
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { auto_despawn: true })
            //make laser fall down
            .insert(Velocity { x: 0., y: -1. });
    }
}
