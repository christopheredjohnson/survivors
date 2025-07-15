use bevy::prelude::*;

use crate::level::{IsUpgradeMenuOpen, LevelUpEvent};

mod enemy;
mod level;
mod player;
mod projectile;
mod ui;
mod upgrade;
mod upgrade_menu;
mod weapon;
mod xp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Vampire Survivors Clone".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_event::<LevelUpEvent>()
        .insert_resource(IsUpgradeMenuOpen::default())
        .insert_resource(level::PlayerXP::default())
        .insert_resource(weapon::WeaponTimer::default())
        .insert_resource(weapon::WeaponStats::default())
        .insert_resource(enemy::EnemySpawnTimer::default())
        .add_systems(Startup, (setup, ui::setup_xp_bar))
        .add_systems(
            Update,
            (
                player::player_movement,
                weapon::weapon_system,
                projectile::projectile_movement,
                enemy::enemy_movement,
                enemy::enemy_spawner,
                projectile::projectile_enemy_collision,
                xp::xp_collection,
                upgrade_menu::show_upgrade_menu,
                upgrade_menu::handle_upgrade_selection,
                ui::update_xp_bar,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    player::spawn_player(&mut commands);
}
