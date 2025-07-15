use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerXP {
    pub current: u32,
    pub level: u32,
    pub required: u32, // XP needed for next level
    pub orb_value: u32,
}

impl Default for PlayerXP {
    fn default() -> Self {
        Self {
            current: 0,
            level: 1,
            required: 10,
            orb_value: 1,
        }
    }
}

#[derive(Event)]
pub struct LevelUpEvent;

#[derive(Resource, Default)]
pub struct IsUpgradeMenuOpen(pub bool);
