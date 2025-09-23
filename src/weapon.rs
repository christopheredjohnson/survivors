use crate::enemy::Enemy;
use crate::player::Player;
use crate::projectile::{Projectile, ProjectileKind};
use bevy::prelude::*;

#[derive(Resource)]
pub struct WeaponTimer(pub Timer);

impl Default for WeaponTimer {
    fn default() -> Self {
        WeaponTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WeaponStats {
    pub multishot: u32,  // Number of projectiles
    pub spread_deg: f32, // Total arc of spread
    pub projectile_speed: f32,
    pub current_shot_type: ProjectileKind,
}

impl Default for WeaponStats {
    fn default() -> Self {
        Self {
            multishot: 1,
            spread_deg: 10.0,
            projectile_speed: 300.0,
            current_shot_type: ProjectileKind::Normal,
        }
    }
}

pub fn weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<WeaponTimer>,
    stats: Res<WeaponStats>,
    player_q: Query<&Transform, With<Player>>,
    enemy_q: Query<&Transform, With<Enemy>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let player_transform = player_q.single();

        let mut nearest: Option<Vec2> = None;
        let mut min_dist_sq = f32::MAX;

        for enemy_transform in enemy_q.iter() {
            let dir = (enemy_transform.translation - player_transform.translation).truncate();
            let dist_sq = dir.length_squared();
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest = Some(dir);
            }
        }

        let base_direction = nearest
            .map(|dir| dir.normalize_or_zero())
            .unwrap_or(Vec2::X);

        let count = stats.multishot.max(1); // prevent 0
        let spread = stats.spread_deg;
        let step = if count > 1 {
            spread.to_radians() / (count - 1) as f32
        } else {
            0.0
        };

        for i in 0..count {
            let offset = step * (i as f32 - (count - 1) as f32 / 2.0);
            let rot = Quat::from_rotation_z(offset);
            let rotated = rot.mul_vec3(base_direction.extend(0.0)).truncate();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(player_transform.translation),
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.7, 0.3),
                        custom_size: Some(Vec2::splat(10.)),
                        ..default()
                    },
                    ..default()
                },
                Projectile {
                    direction: rotated.normalize_or_zero(),
                    kind: stats.current_shot_type,
                },
            ));
        }
    }
}
