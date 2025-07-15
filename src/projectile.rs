use crate::enemy::Enemy;
use crate::weapon::WeaponStats;
use crate::xp;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub kind: ProjectileKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProjectileKind {
    Normal,
    Fireball,
    Ice,
    Piercing,
}

pub fn projectile_movement(
    mut q: Query<(&mut Transform, &Projectile)>,
    stats: Res<WeaponStats>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in q.iter_mut() {
        transform.translation +=
            (projectile.direction * stats.projectile_speed * time.delta_seconds()).extend(0.0);
    }
}

pub fn projectile_enemy_collision(
    mut commands: Commands,
    projectile_q: Query<(Entity, &Transform, &Projectile)>,
    enemy_q: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (proj_e, proj_t, proj) in projectile_q.iter() {
        for (enemy_e, enemy_t) in enemy_q.iter() {
            if proj_t.translation.distance(enemy_t.translation) < 20.0 {
                match proj.kind {
                    ProjectileKind::Normal => {
                        xp::spawn_xp(&mut commands, enemy_t.translation);
                        commands.entity(enemy_e).despawn();
                        commands.entity(proj_e).despawn();
                    }
                    ProjectileKind::Fireball => {
                        // AoE effect: despawn all nearby enemies
                        for (e, t) in enemy_q.iter() {
                            if t.translation.distance(proj_t.translation) < 50.0 {
                                xp::spawn_xp(&mut commands, t.translation);
                                commands.entity(e).despawn();
                            }
                        }
                        commands.entity(proj_e).despawn();
                    }
                    ProjectileKind::Piercing => {
                        xp::spawn_xp(&mut commands, enemy_t.translation);
                        // Damage multiple enemies; don't despawn projectile
                        commands.entity(enemy_e).despawn();
                    }
                    ProjectileKind::Ice => {
                        xp::spawn_xp(&mut commands, enemy_t.translation);
                        // TODO: Slow effect (can be added with a Slowed component + timer)
                        commands.entity(enemy_e).despawn();
                        commands.entity(proj_e).despawn();
                    }
                }
                break;
            }
        }
    }
}
