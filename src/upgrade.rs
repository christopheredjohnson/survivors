use bevy::prelude::*;

use crate::projectile::ProjectileKind;

#[derive(Clone)]
pub enum UpgradeEffect {
    IncreaseMultishot(u32),
    IncreaseSpread(f32),
    IncreaseProjectileSpeed(f32),
    IncreaseMoveSpeed(f32),
    IncreaseXPGain(u32),
    ChangeShotType(ProjectileKind),
}

#[derive(Component)]
pub struct UpgradeButton(pub UpgradeEffect);

#[derive(Component)]
pub struct UpgradeMenuRoot;
