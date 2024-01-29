use bevy::prelude::*;

use crate::{actions::Actions, loading::TextureAssets};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(textures: Res<TextureAssets>) -> Self {
        Self {
            marker: Player,
            sprite: SpriteBundle {
                texture: textures.bevy.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 2.),
                    // scale: Vec3::new(1920., 1080., 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    // color: Color::rgb(0.3, 0.3, 0.7),
                    ..default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 300.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
