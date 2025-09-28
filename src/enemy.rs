use std::time::Duration;

use crate::{health::{DamageEvent, Health}, player::Player};
use bevy::prelude::*;
use rand::Rng;


#[derive(Reflect)]
pub enum EnemyType {
    Skeleton,
    Orc,
    Werewolf
}
#[derive(Clone)]
pub struct EnemyDefinition {
    pub name: &'static str,
    pub texture_path: &'static str,
    pub frame_size: UVec2,
    pub columns: u32,
    pub rows: u32,
    pub scale: f32,
    pub speed: f32,
}

impl EnemyType {
    pub fn definition(&self) -> EnemyDefinition {
        match self {
            EnemyType::Skeleton => EnemyDefinition {
                name: "Skeleton",
                texture_path: "Skeleton.png",
                frame_size: UVec2::splat(100),
                columns: 8,
                rows: 7,
                scale: 1.5,
                speed: 50.0,
            },
            EnemyType::Orc => EnemyDefinition {
                name: "Orc",
                texture_path: "Orc.png",
                frame_size: UVec2::splat(100),
                columns: 8,
                rows: 7,
                scale: 1.5,
                speed: 100.0,
            },
            EnemyType::Werewolf => EnemyDefinition {
                name: "Werewolf",
                texture_path: "Werewolf.png",
                frame_size: UVec2::splat(100),
                columns: 8,
                rows: 7,
                scale: 1.5,
                speed: 150.0,
            },
        }
    }
}

#[derive(Component, Reflect)]
pub struct Enemy {
    pub speed: f32,
    pub enemy_type: EnemyType,
}

#[derive(Resource, Reflect)]
pub struct EnemySpawnTimer(Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.25, TimerMode::Repeating))
    }
}

pub fn enemy_movement(
    mut enemies: Query<(&mut Transform, &Enemy),  Without<Player>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let player_transform = player.single();
    for (mut transform, enemy) in enemies.iter_mut() {
        let dir = (player_transform.translation - transform.translation)
            .truncate()
            .normalize_or_zero();
        transform.translation += (dir * enemy.speed * time.delta_seconds()).extend(0.0);
    }
}

pub fn difficulty_scaling(mut timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds();
    let new_interval = (0.25 + (100.0 / (elapsed + 100.0))).max(0.1);
    timer.0.set_duration(Duration::from_secs_f32(new_interval));
}

pub fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    player_q: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let player_t = player_q.single();
        let mut rng = rand::thread_rng();

        // random enemy type
        let enemy_type = match rng.gen_range(0..3) {
            0 => EnemyType::Skeleton,
            1 => EnemyType::Orc,
            _ => EnemyType::Werewolf,
        };

        let def = enemy_type.definition();

        // Spawn enemies in a ring around the player
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(300.0..400.0);
        let spawn_x = player_t.translation.x + radius * angle.cos();
        let spawn_y = player_t.translation.y + radius * angle.sin();

        let texture = asset_server.load(def.texture_path);

        let layout = TextureAtlasLayout::from_grid(def.frame_size, def.columns, def.rows, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.0).with_scale(Vec3::splat(def.scale)),
                texture: texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            Enemy {
                speed: def.speed,
                enemy_type,
            },
            Health::new(100.0),
            Name::new(def.name),
        ));
    }
}

pub fn enemy_player_collision(
    mut damage_writer: EventWriter<DamageEvent>,
    player_q: Query<(Entity, &Transform), With<Player>>,
    enemy_q: Query<&Transform, With<Enemy>>,
) {
    let (player_e, player_transform) = player_q.single();

    for enemy_t in enemy_q.iter() {
        if player_transform.translation.distance(enemy_t.translation) < 20.0 {
            damage_writer.send(DamageEvent {
                entity: player_e,
                amount: 10.0,
            });
        }
    }
}
