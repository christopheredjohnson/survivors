use bevy::prelude::*;

use crate::{enemy::Enemy, level, player::Player};


#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,   // Who should take the damage
    pub amount: f32,      // How much damage
}

#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
}

#[derive(Component, Reflect)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

pub fn apply_damage_system(
    mut damage_events: EventReader<DamageEvent>,
    mut death_writer: EventWriter<DeathEvent>,
    mut health_q: Query<&mut Health>,
) {
    for ev in damage_events.read() {
        if let Ok(mut health) = health_q.get_mut(ev.entity) {
            health.current -= ev.amount;

            if health.current <= 0.0 {
                death_writer.send(DeathEvent { entity: ev.entity });
                // Don't despawn here — leave that for the death system
            }
        }
    }
}

pub fn enemy_death_system(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    enemy_q: Query<&Transform, With<Enemy>>,
) {
    for ev in death_events.read() {
        if let Ok(transform) = enemy_q.get(ev.entity) {
            level::spawn_xp(&mut commands, transform.translation);
            commands.entity(ev.entity).despawn();
            println!("Enemy died, dropped XP!");
        }
    }
}

pub fn player_death_system(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    player_q: Query<Entity, With<Player>>,
) {
    for ev in death_events.read() {
        if player_q.get(ev.entity).is_ok() {
            println!("Player died! Game Over.");
            // TODO: show game over UI, pause game, etc.
            commands.entity(ev.entity).despawn();
        }
    }
}
