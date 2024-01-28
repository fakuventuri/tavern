use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct IngamePlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct OnIngameScreen;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    move_player.run_if(in_state(GameState::Playing)),
                    esc_to_menu.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>);
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(OnIngameScreen);
}

fn move_player(
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

fn esc_to_menu(mut keys: ResMut<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        keys.reset(KeyCode::Escape);
    }
}
