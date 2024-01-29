use crate::loading::TextureAssets;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

pub struct IngamePlugin;

const CAMERA_SPEED: f32 = 600.;

#[derive(Component)]
pub struct OnIngameScreen;

#[derive(Debug, Clone, Copy, PartialEq)]
enum InteractibleAction {
    SeeBar,
    ExitBar,
    _None,
}

#[derive(Component, Debug)]
struct Interactible {
    action: InteractibleAction,
}

#[derive(Resource)]
struct ActiveInteractibleActions(Vec<InteractibleAction>);

/// IngamePlugin logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(ActiveInteractibleActions(vec![InteractibleAction::SeeBar]))
            .add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            .add_systems(
                Update,
                (
                    esc_to_pause.run_if(in_state(GameState::Playing)),
                    interactibles_system.run_if(in_state(GameState::Playing)),
                    move_camera_system.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>);
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct MoveCameraTo(Option<Vec2>);

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
        .insert(MoveCameraTo(None))
        .insert(OnIngameScreen);
}

#[derive(Component)]
struct CameraBound(Vec2);

fn setup_ingame(mut commands: Commands, textures: Res<TextureAssets>) {
    // CameraBounds Black Sprite out of screen to hide sprites out of view with weird resolutions. // ToDo look for a better solution
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
        .insert(CameraBound(Vec2::new(0., -1080.)))
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
        .insert(CameraBound(Vec2::new(0., 1080.)))
        .insert(OnIngameScreen);
    // Left of the Screen
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-1920., 0., 666.),
                scale: Vec3::new(1920., 1080., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                ..default()
            },
            ..Default::default()
        })
        .insert(CameraBound(Vec2::new(-1920., 0.)))
        .insert(OnIngameScreen);
    // Right of the Screen
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(1920., 0., 666.),
                scale: Vec3::new(1920., 1080., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                ..default()
            },
            ..Default::default()
        })
        .insert(CameraBound(Vec2::new(1920., 0.)))
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
            action: InteractibleAction::SeeBar,
        });
}

fn move_camera_system(
    mut camera_q: Query<
        (&mut Transform, &mut MoveCameraTo),
        (With<MainCamera>, Without<CameraBound>),
    >,
    mut bounds_q: Query<(&mut Transform, &CameraBound), Without<MainCamera>>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut move_camera_to) = camera_q.single_mut();

    // info!("{:?}", move_camera_to.0);

    if let Some(target_pos) = move_camera_to.0 {
        let target = target_pos.extend(0.);
        let current_position = camera_transform.translation;
        if current_position.distance(target) > 5. {
            camera_transform.translation += (target - current_position).normalize_or_zero()
                * CAMERA_SPEED
                * time.delta_seconds();
        } else {
            camera_transform.translation.x = target.x;
            camera_transform.translation.y = target.y;
            move_camera_to.0 = None;
        }
    }

    // Adjust CameraBounds
    for (mut bound_transform, bound) in bounds_q.iter_mut() {
        bound_transform.translation =
            (bound.0 + camera_transform.translation.truncate()).extend(666.);
    }
}

fn interactibles_system(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<
        (&Camera, &GlobalTransform, &mut MoveCameraTo),
        (With<MainCamera>, Without<Interactible>),
    >,
    mut interactibles_q: Query<(&mut Interactible, &Transform, &Handle<Image>, &mut Sprite)>,
    mut active_interactibles: ResMut<ActiveInteractibleActions>,
    assets: Res<Assets<Image>>,
    buttons: Res<Input<MouseButton>>,
) {
    let (camera, camera_global_transform, mut move_camera_to) = camera_q.single_mut();

    if let Some(cursor_world_position) = windows_q
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global_transform, cursor))
    {
        // Cursor is inside the primary window, at 'world_position'

        // Sort interactibles by Z index to interact only with the higher one
        let mut interactibles = interactibles_q.iter_mut().collect::<Vec<_>>();
        interactibles.sort_by(|a, b| b.1.translation.z.total_cmp(&a.1.translation.z));

        let mut found_collision = false;

        for (
            mut interactible,
            interactible_transform,
            interactible_image_handle,
            mut interactible_sprite,
        ) in interactibles
        {
            if found_collision {
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

                // Set bool to ignore the other interactibles
                found_collision = true;

                if !active_interactibles.0.contains(&interactible.action) {
                    interactible_sprite.color = Color::rgb(1., 1., 1.);
                    continue;
                }

                // Highlight
                interactible_sprite.color = Color::rgb(1.3, 1.3, 1.3);

                // Handle mouse click
                if buttons.just_pressed(MouseButton::Left) {
                    // Left button was pressed
                    // info!("Clicked: {:?}", interactible.action);
                    handle_interactible_click(
                        interactible.as_mut(),
                        move_camera_to.as_mut(),
                        active_interactibles.as_mut(),
                    );
                }
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

fn handle_interactible_click(
    interactible: &mut Interactible,
    move_camera_to: &mut MoveCameraTo,
    active_interactibles: &mut ActiveInteractibleActions,
) {
    match interactible.action {
        InteractibleAction::SeeBar => {
            move_camera_to.0 = Some(Vec2::new(0., -215.)); // -230.
                                                           // Deactivate SeeBar
            active_interactibles.0.swap_remove(
                active_interactibles
                    .0
                    .iter()
                    .position(|x| *x == InteractibleAction::SeeBar)
                    .expect("InteractibleAction to remove is not active."),
            );
            // Activate ExitBar
            active_interactibles.0.push(InteractibleAction::ExitBar);
        }
        InteractibleAction::ExitBar => {
            move_camera_to.0 = Some(Vec2::new(0., 0.));
            // Deactivate ExitBar
            active_interactibles.0.swap_remove(
                active_interactibles
                    .0
                    .iter()
                    .position(|x| *x == InteractibleAction::ExitBar)
                    .expect("InteractibleAction to remove is not active."),
            );
            // Activate SeeBar
            active_interactibles.0.push(InteractibleAction::SeeBar);
        }
        InteractibleAction::_None => {}
    }
}

// ToDo change from go to main menu to pause state ingame (create IngameState too)
fn esc_to_pause(mut keys: ResMut<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
        keys.reset(KeyCode::Escape);
    }
}
