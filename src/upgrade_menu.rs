use crate::level::{IsUpgradeMenuOpen, LevelUpEvent, PlayerXP};
use crate::projectile::ProjectileKind;
use crate::upgrade::{UpgradeButton, UpgradeEffect, UpgradeMenuRoot};
use crate::weapon::WeaponStats;
use bevy::color::palettes::css;
use bevy::prelude::*;
use rand::seq::SliceRandom;

pub fn show_upgrade_menu(
    mut commands: Commands,
    mut ev_levelup: EventReader<LevelUpEvent>,
    mut open: ResMut<IsUpgradeMenuOpen>,
    asset_server: Res<AssetServer>,
) {
    if ev_levelup.is_empty() || open.0 {
        return;
    }
    ev_levelup.clear();
    open.0 = true;

    let all_upgrades = vec![
        UpgradeEffect::IncreaseMultishot(1),
        UpgradeEffect::IncreaseSpread(10.0),
        UpgradeEffect::IncreaseProjectileSpeed(100.0),
        UpgradeEffect::IncreaseMoveSpeed(50.0),
        UpgradeEffect::IncreaseXPGain(1),
        UpgradeEffect::ChangeShotType(ProjectileKind::Fireball),
        UpgradeEffect::ChangeShotType(ProjectileKind::Ice),
        UpgradeEffect::ChangeShotType(ProjectileKind::Piercing),
    ];

    let mut rng = rand::thread_rng();
    let selected = all_upgrades
        .choose_multiple(&mut rng, 3)
        .cloned()
        .collect::<Vec<_>>();

    commands
        .spawn(NodeBundle {
            style: Style {
                // size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        })
        .insert(UpgradeMenuRoot)
        .with_children(|parent| {
            for upgrade in selected {
                let label = match &upgrade {
                    UpgradeEffect::ChangeShotType(kind) => match kind {
                        ProjectileKind::Fireball => "Fireball Shot".to_string(),
                        ProjectileKind::Ice => "Ice Shot".to_string(),
                        ProjectileKind::Piercing => "Piercing Shot".to_string(),
                        ProjectileKind::Normal => "Normal Shot".to_string(),
                    },
                    UpgradeEffect::IncreaseMultishot(n) => format!("+{} Multishot", n),
                    UpgradeEffect::IncreaseSpread(s) => format!("+{}° Spread", s),
                    UpgradeEffect::IncreaseProjectileSpeed(s) => format!("+{} Shot Speed", s),
                    UpgradeEffect::IncreaseMoveSpeed(s) => format!("+{} Move Speed", s),
                    UpgradeEffect::IncreaseXPGain(n) => format!("+{} XP per Orb", n),
                };

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            // size: Size::new(Val::Px(220.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: css::DARK_GRAY.into(),
                        ..default()
                    })
                    .insert(UpgradeButton(upgrade.clone()))
                    .with_children(|p| {
                        p.spawn(TextBundle::from_section(
                            label,
                            TextStyle {
                                //   font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn handle_upgrade_selection(
    mut commands: Commands,
    mut interaction_q: Query<(&Interaction, &UpgradeButton, Entity), Changed<Interaction>>,
    mut stats: ResMut<WeaponStats>,
    mut xp: ResMut<PlayerXP>,
    mut open: ResMut<IsUpgradeMenuOpen>,
    upgrade_menu_roots: Query<Entity, With<UpgradeMenuRoot>>,
) {
    for (interaction, button, _entity) in interaction_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            match &button.0 {
                UpgradeEffect::ChangeShotType(kind) => {
                    stats.current_shot_type = *kind;
                }
                UpgradeEffect::IncreaseMultishot(n) => stats.multishot += n,
                UpgradeEffect::IncreaseSpread(s) => stats.spread_deg += s,
                UpgradeEffect::IncreaseProjectileSpeed(s) => stats.projectile_speed += s,
                UpgradeEffect::IncreaseMoveSpeed(_) => println!("TODO: Move speed upgrade"),
                UpgradeEffect::IncreaseXPGain(x) => xp.orb_value += x,
            }

            let menu_roots = upgrade_menu_roots.iter();
            for e in menu_roots {
                commands.entity(e).despawn_recursive();
            }

            open.0 = false;
        }
    }
}
