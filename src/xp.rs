use bevy::prelude::*;

use crate::{
    level::{LevelUpEvent, PlayerXP},
    player::Player,
};

#[derive(Component)]
pub struct XP;

pub fn spawn_xp(commands: &mut Commands, pos: Vec3) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(8.0)),
            sprite: Sprite {
                color: Color::rgb(0.2, 0.6, 1.0),
                ..default()
            },
            ..default()
        },
        XP,
    ));
}

pub fn xp_collection(
    mut commands: Commands,
    mut param: ParamSet<(
        Query<(Entity, &mut Transform), With<XP>>,
        Query<&Transform, With<Player>>,
    )>,
    mut xp_resource: ResMut<PlayerXP>,
    mut level_up_writer: EventWriter<LevelUpEvent>, // ✅ Don't forget this
    time: Res<Time>,
) {
    let player_query = param.p1();
    let player_t = player_query.single();
    let player_pos = player_t.translation.truncate();

    for (xp_e, mut xp_t) in param.p0().iter_mut() {
        let xp_pos = xp_t.translation.truncate();
        let dist = xp_pos.distance(player_pos);

        if dist < 20.0 {
            commands.entity(xp_e).despawn();
            xp_resource.current += xp_resource.orb_value;

            if xp_resource.current >= xp_resource.required {
                xp_resource.current -= xp_resource.required;
                xp_resource.level += 1;
                xp_resource.required = (xp_resource.required as f32 * 1.5).ceil() as u32;

                println!(
                    "🎉 Level Up! Level {}, Next: {}",
                    xp_resource.level, xp_resource.required
                );
                level_up_writer.send(LevelUpEvent); // ✅ send the event
            }
        } else if dist < 150.0 {
            let direction = (player_pos - xp_pos).normalize_or_zero();
            let speed = 200.0;
            xp_t.translation += (direction * speed * time.delta_seconds()).extend(0.0);
        }
    }
}
