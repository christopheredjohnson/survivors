use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(32.)),
            sprite: Sprite {
                color: Color::rgb(0.2, 0.8, 0.2),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

pub fn player_movement(
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
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

    let speed = 200.0;
    transform.translation += (dir.normalize_or_zero() * speed * time.delta_seconds()).extend(0.0);
}
