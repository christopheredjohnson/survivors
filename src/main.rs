use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{enemy::{EnemySpawnTimer}, health::{DamageEvent, DeathEvent, Health}, level::XPPlugin, player::PlayerPlugin, projectile::Projectile};

mod enemy;
mod level;
mod player;
mod projectile;
mod ui;
mod upgrade;
mod upgrade_menu;
mod weapon;
mod health;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Vampire Survivors Clone".to_string(),
                        resolution: (800., 600.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        .add_plugins((PlayerPlugin, XPPlugin, WorldInspectorPlugin::default()))
        .insert_resource(weapon::WeaponTimer::default())
        .insert_resource(weapon::WeaponStats::default())
        .register_type::<Health>()
        .register_type::<Projectile>()
        .register_type::<EnemySpawnTimer>()
        .insert_resource(enemy::EnemySpawnTimer::default())
        .add_systems(Startup, (
            
            setup, 
            ui::setup_xp_bar,
          
        ))
        .add_systems(
            Update,
            (
                weapon::weapon_system,
                projectile::projectile_movement,
                enemy::enemy_movement,
                enemy::difficulty_scaling,
                enemy::enemy_spawner,
                projectile::projectile_enemy_collision,
                upgrade_menu::show_upgrade_menu,
                upgrade_menu::handle_upgrade_selection,
                ui::update_xp_bar,
                enemy::enemy_player_collision,
                health::apply_damage_system,
                health::enemy_death_system,
                health::player_death_system,
                health::tick_damage_cooldown,
                health::spawn_health_bar,
                health::update_health_bars,
                projectile::projectile_bounds_cleanup,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
