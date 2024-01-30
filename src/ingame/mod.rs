mod bar;
mod customer;
mod pause_menu;
use crate::loading::TextureAssets;
use crate::menu::settings::{setting_button_handle, settings_button_colors, OnSettingsMenuScreen};
use crate::{despawn_screen, remove_value_from_vec, GameState, ScreenMode};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use self::bar::{BarPlugin, Drink};
use self::pause_menu::{handle_button, settings_pause_setup, setup_pause_menu, OnPauseMenu};

pub struct IngamePlugin;

const CAMERA_SPEED: f32 = 650.;

#[derive(Component)]
pub struct OnIngameScreen;

#[derive(Component)]
struct OnPauseUI;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum IngameState {
    Running,
    Paused,
    Settings,
    ToMenu,
    #[default]
    Diabled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InteractibleAction {
    EnterBar,
    ExitBar,
    Barrel(Drink),
    Customer,
    _None,
}

impl InteractibleAction {
    fn get_barrels() -> Vec<InteractibleAction> {
        Drink::iterator()
            .map(|(drink, _)| InteractibleAction::Barrel(drink))
            .collect()
    }
}

#[derive(Component, Debug)]
struct Interactible {
    action: InteractibleAction,
}

#[derive(Component)]
struct ActiveInteractibleActions(Vec<InteractibleAction>);

#[derive(Component)]
struct IgnoredInteractibleActions(Vec<InteractibleAction>);

/// IngamePlugin logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_state::<IngameState>()
            .add_plugins(BarPlugin)
            // GameState::Playing // starts with IngameState::Disabled
            .add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            .add_systems(Update, handle_esc.run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>)
            // IngameState::Running
            .add_systems(OnEnter(IngameState::Running), cursor_grab)
            .add_systems(
                Update,
                (
                    interactibles_system.run_if(in_state(IngameState::Running)),
                    move_camera_system.run_if(in_state(IngameState::Running)),
                ),
            )
            .add_systems(OnExit(IngameState::Running), cursor_ungrab)
            // IngameState::Paused
            .add_systems(OnEnter(IngameState::Paused), setup_pause_menu)
            .add_systems(Update, handle_button.run_if(in_state(IngameState::Paused)))
            .add_systems(OnExit(IngameState::Paused), despawn_screen::<OnPauseMenu>)
            // IngameState::Settings
            .add_systems(OnEnter(IngameState::Settings), settings_pause_setup)
            .add_systems(
                Update,
                (
                    handle_button.run_if(in_state(IngameState::Settings)),
                    setting_button_handle::<ScreenMode>.run_if(in_state(IngameState::Settings)),
                    settings_button_colors.run_if(in_state(IngameState::Settings)),
                ),
            )
            .add_systems(
                OnExit(IngameState::Settings),
                despawn_screen::<OnSettingsMenuScreen>,
            )
            // To Main Menu
            .add_systems(OnEnter(IngameState::ToMenu), go_to_main_menu);
    }
}

#[derive(Component)]
pub struct MainCameraIngame;

#[derive(Component)]
struct MoveCameraTo(Option<Vec2>);

#[derive(Component)]
struct CameraBound(Vec2);

fn setup_camera(mut commands: Commands) {
    // CameraBounds Black Sprites out of screen to hide sprites out of window in weird resolutions.
    // ToDo look for a better solution
    // Bottom of the Screen
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -1080., 999.),
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
                translation: Vec3::new(0., 1080., 999.),
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
                translation: Vec3::new(-1920., 0., 999.),
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
                translation: Vec3::new(1920., 0., 999.),
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

    // Camera
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::AutoMin {
        min_width: 1920.,
        min_height: 1080.,
    };
    // camera_bundle.camera_2d.clear_color =
    //     bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.5, 0.5, 0.5));
    // camera_bundle.camera.hdr = true; // Weir behabior (like a weird effect) with Rgba with high alpha values

    commands
        .spawn(camera_bundle)
        .insert(MainCameraIngame)
        .insert(MoveCameraTo(None))
        .insert(OnIngameScreen);
}

fn setup_ingame(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut ingame_state: ResMut<NextState<IngameState>>,
) {
    // ActiveInteractibleActions
    let mut initial_active_interactibles =
        vec![InteractibleAction::EnterBar, InteractibleAction::Customer];
    initial_active_interactibles.append(&mut InteractibleAction::get_barrels());
    commands
        .spawn(ActiveInteractibleActions(initial_active_interactibles))
        .insert(OnIngameScreen);
    // IgnoredInteractibleActions
    let mut initial_ignored_interactibles = vec![];
    initial_ignored_interactibles.append(&mut InteractibleAction::get_barrels());
    commands
        .spawn(IgnoredInteractibleActions(initial_ignored_interactibles))
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

    // Set game state to Running to start systems
    ingame_state.set(IngameState::Running)
}

fn move_camera_system(
    mut camera_q: Query<
        (&mut Transform, &mut MoveCameraTo),
        (With<MainCameraIngame>, Without<CameraBound>),
    >,
    mut bounds_q: Query<(&mut Transform, &CameraBound), Without<MainCameraIngame>>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut move_camera_to) = camera_q.single_mut();

    if let Some(target_pos) = move_camera_to.0 {
        // info!("{:?}", move_camera_to.0);
        let target = target_pos.extend(0.);
        let current_position = camera_transform.translation;
        if current_position.distance(target) > 10. {
            camera_transform.translation += (target - current_position).normalize_or_zero()
                * CAMERA_SPEED
                * time.delta_seconds();
        } else {
            camera_transform.translation.x = target.x;
            camera_transform.translation.y = target.y;
            move_camera_to.0 = None;
        }

        // Adjust CameraBounds
        for (mut bound_transform, bound) in bounds_q.iter_mut() {
            bound_transform.translation =
                (bound.0 + camera_transform.translation.truncate()).extend(999.);
        }
    }
}

fn interactibles_system(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<
        (&Camera, &GlobalTransform, &mut MoveCameraTo),
        (With<MainCameraIngame>, Without<Interactible>),
    >,
    mut interactibles_q: Query<(&Transform, &mut Interactible, &Handle<Image>, &mut Sprite)>,
    mut active_interactibles_q: Query<&mut ActiveInteractibleActions>,
    mut ignored_interactibles_q: Query<&mut IgnoredInteractibleActions>,
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

        // Active Interactibles
        let mut active_interactibles = active_interactibles_q.single_mut();
        let mut ignored_interactibles = ignored_interactibles_q.single_mut();
        // Sort interactibles by Z index to interact only with the higher one
        let mut interactibles = interactibles_q.iter_mut().collect::<Vec<_>>();
        interactibles.sort_by(|a, b| b.0.translation.z.total_cmp(&a.0.translation.z));

        let mut found_collision = false;

        for (
            interactible_transform,
            mut interactible,
            interactible_image_handle,
            mut interactible_sprite,
        ) in interactibles
        {
            if ignored_interactibles.0.contains(&interactible.action) || found_collision {
                interactible_sprite.color = Color::rgb(1., 1., 1.);
                continue;
            }
            // Calculate Interactible Size by the image.
            let image_dimensions = assets.get(interactible_image_handle).unwrap().size();
            let scaled_image_dimension =
                image_dimensions.as_vec2() * interactible_transform.scale.truncate();

            // Calculate interactible translation for collision
            let mut interacticle_translation = interactible_transform.translation;
            if interactible.action == InteractibleAction::Customer {
                interacticle_translation.y =
                    interacticle_translation.y + scaled_image_dimension.y / 2.;
            }

            if let Some(_collision) = collide(
                interacticle_translation,
                scaled_image_dimension,
                cursor_world_position.extend(0.),
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
                    handle_interactible_click(
                        interactible.as_mut(),
                        move_camera_to.as_mut(),
                        active_interactibles.as_mut(),
                        ignored_interactibles.as_mut(),
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
            _interactible_transform,
            _interactible,
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
    ignored_interactibles: &mut IgnoredInteractibleActions,
) {
    match interactible.action {
        InteractibleAction::EnterBar => {
            move_camera_to.0 = Some(Vec2::new(0., -630.)); // -275. = One shelf height | -630. = Two shelf height

            // Deactivate EnterBar
            remove_value_from_vec(InteractibleAction::EnterBar, &mut active_interactibles.0);
            // Ignore Customer
            ignored_interactibles.0.push(InteractibleAction::Customer);
            // Stop ignoring Barrels
            InteractibleAction::get_barrels()
                .iter()
                .for_each(|barrel| remove_value_from_vec(*barrel, &mut ignored_interactibles.0));
            // Activate ExitBar
            active_interactibles.0.push(InteractibleAction::ExitBar);
        }
        InteractibleAction::ExitBar => {
            move_camera_to.0 = Some(Vec2::new(0., 0.));
            // Deactivate ExitBar
            remove_value_from_vec(InteractibleAction::ExitBar, &mut active_interactibles.0);
            // Ignore Barrels
            InteractibleAction::get_barrels()
                .iter()
                .for_each(|barrel| ignored_interactibles.0.push(*barrel));
            // Stop ignoring Customer
            remove_value_from_vec(InteractibleAction::Customer, &mut ignored_interactibles.0);
            // Activate EnterBar
            active_interactibles.0.push(InteractibleAction::EnterBar);
        }
        InteractibleAction::Barrel(drink) => {
            info!("Clicked Barrel: {:?}", drink);
        }
        InteractibleAction::Customer => {}
        InteractibleAction::_None => {}
    }
}

fn handle_esc(
    mut keys: ResMut<Input<KeyCode>>,
    ingame_state: Res<State<IngameState>>,
    mut ingame_next_state: ResMut<NextState<IngameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.reset(KeyCode::Escape);
        match *ingame_state.get() {
            IngameState::Running => ingame_next_state.set(IngameState::Paused),
            IngameState::Paused => ingame_next_state.set(IngameState::Running),
            IngameState::Settings => ingame_next_state.set(IngameState::Paused),
            _ => {}
        }
    }
}

/// Grab Cursor to prevent it from leaving the window
fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // Use the cursor, but not let it leave the window.
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;
}

/// Release Cursor
fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
}

fn go_to_main_menu(mut game_next_state: ResMut<NextState<GameState>>) {
    game_next_state.set(GameState::Menu);
}
