use bevy::{color::palettes::css, prelude::*};

use crate::level::PlayerXP;

#[derive(Component)]
pub struct XpBarFill; // Tag for the "fill" part of the bar

pub fn setup_xp_bar(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0), // ✅ set individually
                left: Val::Px(10.0),
                right: Val::Px(10.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: css::DARK_GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0), // fill starts empty
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.7, 1.0).into(),
                    ..default()
                })
                .insert(XpBarFill);
        });
}

pub fn update_xp_bar(xp: Res<PlayerXP>, mut query: Query<&mut Style, With<XpBarFill>>) {
    if !xp.is_changed() {
        return;
    }

    let percent = (xp.current as f32 / xp.required as f32).clamp(0.0, 1.0);
    for mut style in query.iter_mut() {
        style.width = Val::Percent(percent * 100.0);
    }
}
