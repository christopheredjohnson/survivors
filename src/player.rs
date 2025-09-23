use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Resource)]
pub struct PlayerStats {
    pub move_speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { move_speed: 200.0 }
    }
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerStats::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}




fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(32.)),
            sprite: Sprite {
                color: Color::srgb(0.2, 0.8, 0.2),
                ..default()
            },
            ..default()
        },
        Player,
        Name::new("Player"),
    ));
}

pub fn player_movement(
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    stats: Res<PlayerStats>,
) {
    let mut transform = q.single_mut();
    let mut dir = Vec2::ZERO;
    if kb.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if kb.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if kb.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if kb.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    transform.translation += (dir.normalize_or_zero() * stats.move_speed * time.delta_seconds()).extend(0.0);
}
