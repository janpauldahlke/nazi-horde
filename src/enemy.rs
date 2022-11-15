use crate::{
    components::{Enemy, SpriteSize},
    EnemyCount, GameTextures, WinSize, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE,
};
use bevy::{prelude::*, transform};
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
        app.add_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // compute random position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 200.;
        let h_span = win_size.h / 2. - 200.;

        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

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
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}
