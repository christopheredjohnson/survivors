use crate::player::Player;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.25, TimerMode::Repeating))
    }
}

pub fn enemy_movement(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for mut transform in enemies.iter_mut() {
        let dir = (player_transform.translation - transform.translation)
            .truncate()
            .normalize_or_zero();
        transform.translation += (dir * 100.0 * time.delta_seconds()).extend(0.0);
    }
}

pub fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    player_q: Query<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let player_t = player_q.single();
        let mut rng = rand::thread_rng();

        // Spawn enemies in a ring around the player
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(300.0..400.0);
        let spawn_x = player_t.translation.x + radius * angle.cos();
        let spawn_y = player_t.translation.y + radius * angle.sin();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.0).with_scale(Vec3::splat(24.0)),
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    ..default()
                },
                ..default()
            },
            Enemy,
        ));
    }
}
