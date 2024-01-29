use crate::loading::TextureAssets;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct IngamePlugin;

#[derive(Component)]
pub struct OnIngameScreen;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            .add_systems(Update, (esc_to_pause.run_if(in_state(GameState::Playing)),))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>);
    }
}

fn setup_camera(mut commands: Commands) {
    // Camera
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::AutoMin {
        min_width: 1920.,
        min_height: 1080.,
    };
    // camera_bundle.camera_2d.clear_color =
    //     bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 0.5));

    commands.spawn(camera_bundle).insert(OnIngameScreen);
}

fn setup_ingame(mut commands: Commands, textures: Res<TextureAssets>) {
    // Background
    commands
        .spawn(SpriteBundle {
            texture: textures.tavern_bg.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen);

    // Counter
    commands
        .spawn(SpriteBundle {
            texture: textures.counter.clone(),
            transform: Transform {
                translation: Vec3::new(0., -540., -1.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen);
}

fn esc_to_pause(mut keys: ResMut<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        keys.reset(KeyCode::Escape);
    }
}
