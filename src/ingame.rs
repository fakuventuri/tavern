use crate::loading::TextureAssets;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::PrimaryWindow;

pub struct IngamePlugin;

#[derive(Component)]
pub struct OnIngameScreen;

#[derive(Debug)]
enum InteractibleAction {
    SeeBar,
    ExitBar,
    None,
}

#[derive(Component, Debug)]
struct Interactible {
    active: bool,
    action: InteractibleAction,
}

/// IngamePlugin logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            .add_systems(
                Update,
                (
                    esc_to_pause.run_if(in_state(GameState::Playing)),
                    handle_interaction.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>);
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    // Camera
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::AutoMin {
        min_width: 1920.,
        min_height: 1080.,
    };
    // camera_bundle.camera_2d.clear_color =
    //     bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 0.5));

    commands
        .spawn(camera_bundle)
        .insert(MainCamera)
        .insert(OnIngameScreen);
}

fn setup_ingame(mut commands: Commands, textures: Res<TextureAssets>) {
    // Black Sprite out of screen to hide sprites out of view with weird resolutions. // ToDo look for a better solution
    // Bottom of the Screen
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -1080., 666.),
                scale: Vec3::new(1920., 1080., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                ..default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen);
    // Top of the Screen
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 1080., 666.),
                scale: Vec3::new(1920., 1080., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                ..default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen);

    // Background
    commands
        .spawn(SpriteBundle {
            texture: textures.tavern_bg.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -10.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            active: true,
            action: InteractibleAction::ExitBar,
        });

    // Counter
    commands
        .spawn(SpriteBundle {
            texture: textures.counter.clone(),
            transform: Transform {
                translation: Vec3::new(0., -540., 0.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            active: true,
            action: InteractibleAction::SeeBar,
        });
}

fn handle_interaction(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut interactibles_q: Query<(&Interactible, &Transform, &Handle<Image>, &mut Sprite)>,
    assets: Res<Assets<Image>>,
    // buttons: Res<Input<MouseButton>>,
) {
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_world_position) = windows_q
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // Cursor is inside the primary window, at 'world_position'
        let mut interactibles = interactibles_q.iter_mut().collect::<Vec<_>>();
        interactibles.sort_by(|a, b| b.1.translation.z.total_cmp(&a.1.translation.z));

        let mut found_collision = false;

        for (
            interactible,
            interactible_transform,
            interactible_image_handle,
            mut interactible_sprite,
        ) in interactibles
        {
            if !interactible.active || found_collision {
                interactible_sprite.color = Color::rgb(1., 1., 1.);
                continue;
            }
            // Calculate Interactible Size by the image.
            let image_dimensions = assets.get(interactible_image_handle).unwrap().size();
            let scaled_image_dimension =
                image_dimensions.as_vec2() * interactible_transform.scale.truncate();

            if let Some(_collision) = collide(
                interactible_transform.translation,
                scaled_image_dimension,
                cursor_world_position.extend(1.),
                Vec2::ONE,
            ) {
                // Collision with mouse. Type Collision::Inside
                // Highlight
                interactible_sprite.color = Color::rgb(1.3, 1.3, 1.3);

                found_collision = true;
            } else {
                // Reset Highlight
                interactible_sprite.color = Color::rgb(1., 1., 1.);
            }
        }
    } else {
        // Cursor is not in the game window.
        for (
            _interactible,
            _interactible_transform,
            _interactible_image_handle,
            mut interactible_sprite,
        ) in interactibles_q.iter_mut()
        {
            interactible_sprite.color = Color::rgb(1., 1., 1.);
        }
    }
}

// ToDo change from go to main menu to pause state ingame (create IngameState too)
fn esc_to_pause(mut keys: ResMut<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        keys.reset(KeyCode::Escape);
    }
}
