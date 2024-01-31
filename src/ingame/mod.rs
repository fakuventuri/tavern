mod bar;
mod customer;
mod pause_menu;
use crate::loading::TextureAssets;
use crate::menu::settings::{setting_button_handle, settings_button_colors, OnSettingsMenuScreen};
use crate::{despawn_screen, GameState, ScaleByAssetResolution, ScreenMode};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use self::bar::{BarPlugin, Drink};
use self::customer::CustomerPlugin;
use self::pause_menu::{handle_button, settings_pause_setup, setup_pause_menu, OnPauseMenu};

pub struct IngamePlugin;

const CAMERA_SPEED: f32 = 900.;

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

#[derive(Component)]
pub struct ClickedInteractible;

#[derive(Bundle)]
pub struct InteractibleBundle {
    interactible_action: InteractibleAction,
    interaction_sprite_colors: InteractionSpriteColors,
}

impl InteractibleBundle {
    pub fn new(interactible_action: InteractibleAction) -> Self {
        Self {
            interactible_action,
            interaction_sprite_colors: InteractionSpriteColors {
                ..Default::default()
            },
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractibleAction {
    Bar,
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

#[derive(Component)]
pub struct InteractionSpriteColors {
    normal: Color,
    highlight: Color,
}

impl Default for InteractionSpriteColors {
    fn default() -> Self {
        Self {
            normal: Color::rgb(1., 1., 1.),
            highlight: Color::rgb(1.3, 1.3, 1.3),
        }
    }
}

#[derive(Component)]
struct ActiveInteractibleActions(Vec<InteractibleAction>);

#[derive(Component)]
struct IgnoredInteractibleActions(Vec<InteractibleAction>);

#[derive(Resource)]
struct DrinkInHand(Option<Drink>);

#[derive(Component)]
struct InHandText;

#[derive(Resource)]
#[allow(dead_code)]
struct PlayerStats {
    pub money: f64,
    pub streak: u32,
    pub highest_streak: u32,
    pub reputation_level: u32,
    pub reputation_progress: u32,     // 1 exp = 1 customer
    pub reputation_progress_max: u32, // ToDo fn to get max reputation for current level with a formula
}

#[derive(Component)]
struct MoneyText;

#[derive(Component)]
struct StreakText;

#[derive(Component)]
struct HighestStreakText;

#[derive(Resource)]
struct CustomersStats {
    pub customers_wait_duration: f32,
    pub customers_spawn_gap: std::ops::Range<u64>,
}

// #[derive(Resource)]
// struct Workday {
//     pub timer: Timer,
// }

#[derive(Resource)]
enum CameraPosition {
    Zero,
    OneShelf,
    TwoShelf,
}

impl CameraPosition {
    fn down(&mut self) {
        *self = match *self {
            CameraPosition::Zero => CameraPosition::OneShelf,
            CameraPosition::OneShelf => CameraPosition::TwoShelf,
            CameraPosition::TwoShelf => CameraPosition::TwoShelf,
        }
    }

    fn up(&mut self) {
        *self = match *self {
            CameraPosition::Zero => CameraPosition::Zero,
            CameraPosition::OneShelf => CameraPosition::Zero,
            CameraPosition::TwoShelf => CameraPosition::OneShelf,
        }
    }

    fn to_vec2(&self) -> Vec2 {
        match *self {
            CameraPosition::Zero => Vec2::new(0., 0.),
            CameraPosition::OneShelf => Vec2::new(0., -275.),
            CameraPosition::TwoShelf => Vec2::new(0., -630.),
        }
    }
}

/// IngamePlugin logic is only active during the State `GameState::Playing`
impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_state::<IngameState>()
            .insert_resource(DrinkInHand(None))
            .insert_resource(PlayerStats {
                money: 0.,
                streak: 0,
                highest_streak: 0,
                reputation_level: 0,
                reputation_progress: 0,
                reputation_progress_max: 10,
            })
            .insert_resource(CustomersStats {
                customers_wait_duration: 3.,
                customers_spawn_gap: 0..3,
            })
            .insert_resource(CameraPosition::Zero)
            .add_plugins(BarPlugin)
            .add_plugins(CustomerPlugin)
            // GameState::Playing // starts with IngameState::Disabled
            .add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            .add_systems(
                Update,
                (
                    handle_esc.run_if(in_state(GameState::Playing)),
                    keys_camera_control.run_if(in_state(GameState::Playing)),
                    update_ui_texts.run_if(in_state(GameState::Playing)),
                ),
            )
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>)
            // IngameState::Running
            // .add_systems(OnEnter(IngameState::Running), cursor_grab) // Mouse bug in web
            .add_systems(
                Update,
                (
                    interactibles_system.run_if(in_state(IngameState::Running)),
                    move_camera_system.run_if(in_state(IngameState::Running)),
                ),
            )
            // .add_systems(OnExit(IngameState::Running), cursor_ungrab) // Mouse bug in web
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
    let mut initial_active_interactibles = vec![InteractibleAction::Customer]; // Add InteractibleAction::Bar to reactivate click transitions
    initial_active_interactibles.append(&mut InteractibleAction::get_barrels());
    commands
        .spawn(ActiveInteractibleActions(initial_active_interactibles))
        .insert(OnIngameScreen);
    // IgnoredInteractibleActions
    #[allow(unused_mut)]
    let mut initial_ignored_interactibles = vec![];
    // initial_ignored_interactibles.append(&mut InteractibleAction::get_barrels());
    commands
        .spawn(IgnoredInteractibleActions(initial_ignored_interactibles))
        .insert(OnIngameScreen);
    // Background
    commands
        .spawn(SpriteBundle {
            texture: textures.tavern_bg.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., -10.),
                scale: ScaleByAssetResolution::Res720p.scale(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(InteractibleBundle::new(InteractibleAction::ExitBar));

    // InHandText
    commands
        .spawn(
            TextBundle::from_section(
                "In hand: None",
                TextStyle {
                    font_size: 50.,
                    color: Color::BLACK,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_background_color(Color::rgba(1., 1., 1., 0.1))
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                bottom: Val::Px(0.),
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            }),
        )
        .insert(InHandText)
        .insert(OnIngameScreen);

    // MoneyText
    commands
        .spawn(
            TextBundle::from_section(
                "Money: 0",
                TextStyle {
                    font_size: 50.,
                    color: Color::BLACK,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_background_color(Color::rgba(1., 1., 1., 0.1))
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            }),
        )
        .insert(MoneyText)
        .insert(OnIngameScreen);

    // StreakText
    commands
        .spawn(
            TextBundle::from_section(
                "Streak: 0",
                TextStyle {
                    font_size: 50.,
                    color: Color::BLACK,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_background_color(Color::rgba(1., 1., 1., 0.1))
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(44.),
                top: Val::Px(0.),
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            }),
        )
        .insert(StreakText)
        .insert(OnIngameScreen);

    // HighestStreakText
    commands
        .spawn(
            TextBundle::from_section(
                "Highest Streak: 0",
                TextStyle {
                    font_size: 50.,
                    color: Color::BLACK,
                    ..Default::default()
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_background_color(Color::rgba(1., 1., 1., 0.1))
            .with_style(Style {
                position_type: PositionType::Absolute,
                right: Val::Px(0.),
                top: Val::Px(0.),
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            }),
        )
        .insert(HighestStreakText)
        .insert(OnIngameScreen);

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
    mut commands: Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<
        (&Camera, &GlobalTransform),
        (With<MainCameraIngame>, Without<InteractibleAction>),
    >,
    mut interactibles_q: Query<(
        Entity,
        &Transform,
        &InteractibleAction,
        &Handle<Image>,
        &mut Sprite,
        &InteractionSpriteColors,
    )>,
    active_interactibles_q: Query<&ActiveInteractibleActions>,
    ignored_interactibles_q: Query<&IgnoredInteractibleActions>,
    assets: Res<Assets<Image>>,
    buttons: Res<Input<MouseButton>>,
) {
    let (camera, camera_global_transform) = camera_q.single();

    if let Some(cursor_world_position) = windows_q
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_global_transform, cursor))
    {
        // Cursor is inside the primary window, at 'world_position'

        // Active Interactibles
        let active_interactibles = active_interactibles_q.single();
        let ignored_interactibles = ignored_interactibles_q.single();
        // Sort interactibles by Z index to interact only with the higher one
        let mut interactibles = interactibles_q.iter_mut().collect::<Vec<_>>();
        interactibles.sort_by(|a, b| b.1.translation.z.total_cmp(&a.1.translation.z));

        let mut found_collision = false;

        for (
            entity,
            interactible_transform,
            interactible_action,
            interactible_image_handle,
            mut interactible_sprite,
            interaction_sprite_colors,
        ) in interactibles
        {
            if found_collision || ignored_interactibles.0.contains(interactible_action) {
                interactible_sprite.color = interaction_sprite_colors.normal;
                continue;
            }
            // Calculate Interactible Size by the image.
            let image_dimensions = assets.get(interactible_image_handle).unwrap().size();
            let scaled_image_dimension =
                image_dimensions.as_vec2() * interactible_transform.scale.truncate();

            // Calculate interactible translation for collision
            let mut interacticle_translation = interactible_transform.translation;
            if *interactible_action == InteractibleAction::Customer {
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
                if *interactible_action != InteractibleAction::Customer {
                    found_collision = true;
                }

                if !active_interactibles.0.contains(interactible_action) {
                    interactible_sprite.color = interaction_sprite_colors.normal;
                    continue;
                }

                // Highlight
                interactible_sprite.color = interaction_sprite_colors.highlight;

                // Handle mouse click
                if buttons.just_pressed(MouseButton::Left) {
                    // Left button was pressed
                    commands.entity(entity).insert(ClickedInteractible);
                }
            } else {
                // Reset Highlight
                interactible_sprite.color = interaction_sprite_colors.normal;
            }
        }
    } else {
        // Cursor is not in the game window.
        for (
            _entity,
            _interactible_transform,
            _interactible,
            _interactible_image_handle,
            mut interactible_sprite,
            interaction_sprite_colors,
        ) in interactibles_q.iter_mut()
        {
            interactible_sprite.color = interaction_sprite_colors.normal;
        }
    }
}

fn update_ui_texts(
    mut q_in_hand_text: Query<
        &mut Text,
        (
            With<InHandText>,
            Without<MoneyText>,
            Without<StreakText>,
            Without<HighestStreakText>,
        ),
    >,
    mut q_money_text: Query<
        &mut Text,
        (
            With<MoneyText>,
            Without<InHandText>,
            Without<StreakText>,
            Without<HighestStreakText>,
        ),
    >,
    mut q_streak_text: Query<
        &mut Text,
        (
            With<StreakText>,
            Without<InHandText>,
            Without<MoneyText>,
            Without<HighestStreakText>,
        ),
    >,
    mut q_highest_streak_text: Query<
        &mut Text,
        (
            With<HighestStreakText>,
            Without<InHandText>,
            Without<MoneyText>,
            Without<StreakText>,
        ),
    >,
    drink_in_hand: Res<DrinkInHand>,
    player_stats: Res<PlayerStats>,
) {
    let mut in_hand_text = q_in_hand_text.single_mut();
    if let Some(drink) = drink_in_hand.0 {
        in_hand_text.sections[0].value = format!("In hand: {}", drink);
    } else {
        in_hand_text.sections[0].value = "In hand: None".to_string();
    }

    let mut money_text = q_money_text.single_mut();
    money_text.sections[0].value = format!("Money: {}", player_stats.money);

    let mut streak_text = q_streak_text.single_mut();
    streak_text.sections[0].value = format!("Streak: {}", player_stats.streak);

    let mut highest_streak_text = q_highest_streak_text.single_mut();
    highest_streak_text.sections[0].value =
        format!("Highest Streak: {}", player_stats.highest_streak);
}

fn keys_camera_control(
    keys: ResMut<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut move_camera_to_q: Query<
        &mut MoveCameraTo,
        (With<MainCameraIngame>, Without<InteractibleAction>),
    >,
    mut camera_position: ResMut<CameraPosition>,
) {
    let mut move_camera_to = move_camera_to_q.single_mut();

    if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        camera_position.up();
        move_camera_to.0 = Some(camera_position.to_vec2());
    }
    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        camera_position.down();
        move_camera_to.0 = Some(camera_position.to_vec2());
    }

    if let Some(scroll) = scroll_evr.read().last() {
        if scroll.y < 0. {
            camera_position.down();
            move_camera_to.0 = Some(camera_position.to_vec2());
        } else if scroll.y > 0. {
            camera_position.up();
            move_camera_to.0 = Some(camera_position.to_vec2());
        }
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
#[allow(dead_code)]
/// Grab Cursor to prevent it from leaving the window
fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // Use the cursor, but not let it leave the window.
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;
}

#[allow(dead_code)]
/// Release Cursor
fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor.grab_mode = CursorGrabMode::None;
}

fn go_to_main_menu(mut game_next_state: ResMut<NextState<GameState>>) {
    game_next_state.set(GameState::Menu);
}
