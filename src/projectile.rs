use crate::enemy::Enemy;
use crate::health::{DamageEvent, Health};
use crate::weapon::WeaponStats;
use crate::level;
use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Projectile {
    pub direction: Vec2,
    pub kind: ProjectileKind,
    pub damage: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Reflect)]
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
    mut damage_writer: EventWriter<DamageEvent>,
    projectile_q: Query<(Entity, &Transform, &Projectile)>,
    enemy_q: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (proj_e, proj_t, proj) in projectile_q.iter() {
        for (enemy_e, enemy_t) in enemy_q.iter() {
            if proj_t.translation.distance(enemy_t.translation) < 20.0 {
                match proj.kind {
                    ProjectileKind::Normal => {
                        damage_writer.send(DamageEvent {
                            entity: enemy_e,
                            amount: proj.damage,
                        });
                        commands.entity(proj_e).despawn();
                    }
                    ProjectileKind::Fireball => {
                        for (e, t) in enemy_q.iter() {
                            if t.translation.distance(proj_t.translation) < 50.0 {
                                damage_writer.send(DamageEvent {
                                    entity: e,
                                    amount: proj.damage,
                                });
                            }
                        }
                        commands.entity(proj_e).despawn();
                    }
                    ProjectileKind::Piercing => {
                        damage_writer.send(DamageEvent {
                            entity: enemy_e,
                            amount: proj.damage,
                        });
                        // projectile stays alive
                    }
                    ProjectileKind::Ice => {
                        damage_writer.send(DamageEvent {
                            entity: enemy_e,
                            amount: proj.damage,
                        });
                        commands.entity(proj_e).despawn();
                    }
                }
                break;
            }
        }
    }
}

