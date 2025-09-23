use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    level::XPPlugin,
    player::PlayerPlugin,
};

mod enemy;
mod level;
mod player;
mod projectile;
mod ui;
mod upgrade;
mod upgrade_menu;
mod weapon;

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
        .add_plugins((PlayerPlugin, XPPlugin, WorldInspectorPlugin::default()))
        .insert_resource(weapon::WeaponTimer::default())
        .insert_resource(weapon::WeaponStats::default())
        .insert_resource(enemy::EnemySpawnTimer::default())
        .add_systems(Startup, (setup, ui::setup_xp_bar))
        .add_systems(
            Update,
            (
                weapon::weapon_system,
                projectile::projectile_movement,
                enemy::enemy_movement,
                enemy::enemy_spawner,
                projectile::projectile_enemy_collision,
                upgrade_menu::show_upgrade_menu,
                upgrade_menu::handle_upgrade_selection,
                ui::update_xp_bar,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
