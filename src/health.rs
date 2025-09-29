use bevy::{color::palettes::css, prelude::*};

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

#[derive(Component)]
pub struct DamageCooldown {
    pub timer: Timer,
}

impl DamageCooldown {
    pub fn new(durartion: f32) -> Self {
        Self {
            timer: Timer::from_seconds(durartion, TimerMode::Once),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.timer.finished()
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

pub fn tick_damage_cooldown(
    time: Res<Time>,
    mut query: Query<&mut DamageCooldown, With<Player>>,
) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}


#[derive(Component)]
pub struct HealthBar;

pub fn spawn_health_bar(
    mut commands: Commands,
    q: Query<Entity, Added<Health>>,
) {
    for entity in &q {
        // Create a Node or a sprite as a child
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: css::GREEN.into(),
                        custom_size: Some(Vec2::new(40.0, 5.0)), // initial size
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 40.0, 10.0),
                    ..default()
                },
                HealthBar,
            ));
        });
    }
}

pub fn update_health_bars(
    mut q: Query<(&Health, &Children)>,
    mut bar_q: Query<&mut Sprite, With<HealthBar>>,
) {
    for (health, children) in &mut q {
        let ratio = health.current / health.max;
        for &child in children {
            if let Ok(mut sprite) = bar_q.get_mut(child) {
                if let Some(size) = &mut sprite.custom_size {
                    size.x = 40.0 * ratio.max(0.0); // shrink width based on ratio
                    sprite.color = if ratio > 0.5 {
                        css::GREEN.into()
                    } else if ratio > 0.2 {
                        css::ORANGE.into()
                    } else {
                        css::RED.into()
                    };
                }
            }
        }
    }
}