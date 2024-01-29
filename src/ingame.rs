use crate::loading::TextureAssets;
use crate::menu::{menu_button, ButtonColors};
use crate::{despawn_screen, remove_value_from_vec, GameState};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

pub struct IngamePlugin;

const CAMERA_SPEED: f32 = 650.;

#[derive(Component)]
pub struct OnIngameScreen;

#[derive(Component)]
struct OnPauseUI;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum IngameState {
    Running,
    Paused,
    ToMenu,
    #[default]
    Diabled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InteractibleAction {
    SeeBar,
    ExitBar,
    BeerBarrel,
    _None,
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
            .add_systems(OnEnter(GameState::Playing), (setup_ingame, setup_camera))
            // IngameState::Running
            .add_systems(
                Update,
                (
                    esc_to_pause.run_if(in_state(GameState::Playing)),
                    interactibles_system.run_if(in_state(IngameState::Running)),
                    move_camera_system.run_if(in_state(IngameState::Running)),
                ),
            )
            // IngameState::Paused
            .add_systems(OnEnter(IngameState::Paused), setup_pause_menu)
            .add_systems(Update, handle_button.run_if(in_state(IngameState::Paused)))
            .add_systems(OnExit(IngameState::Paused), despawn_screen::<OnPauseMenu>)
            // To Main Menu
            .add_systems(OnEnter(IngameState::ToMenu), go_to_menu)
            .add_systems(OnExit(GameState::Playing), despawn_screen::<OnIngameScreen>);
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct MoveCameraTo(Option<Vec2>);

fn setup_camera(mut commands: Commands) {
    // CameraBounds Black Sprite out of screen to hide sprites out of view in weird resolutions. // ToDo look for a better solution
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

    commands
        .spawn(camera_bundle)
        .insert(MainCamera)
        .insert(MoveCameraTo(None))
        .insert(OnIngameScreen);
}

#[derive(Component)]
struct CameraBound(Vec2);

fn setup_ingame(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut ingame_state: ResMut<NextState<IngameState>>,
) {
    // ActiveInteractibleActions
    commands
        .spawn(ActiveInteractibleActions(vec![
            InteractibleAction::SeeBar,
            InteractibleAction::BeerBarrel,
        ]))
        .insert(OnIngameScreen);
    // IgnoredInteractibleActions
    commands
        .spawn(IgnoredInteractibleActions(vec![
            InteractibleAction::BeerBarrel,
        ]))
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
                translation: Vec3::new(0., -600., 0.), // y: -540.
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            action: InteractibleAction::SeeBar,
        });

    // BeerBarrels
    commands
        .spawn(SpriteBundle {
            texture: textures.barrel.clone(),
            transform: Transform {
                translation: Vec3::new(750., -615., 4.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            action: InteractibleAction::BeerBarrel,
        });

    commands
        .spawn(SpriteBundle {
            texture: textures.barrel.clone(),
            transform: Transform {
                translation: Vec3::new(400., -615., 3.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            action: InteractibleAction::BeerBarrel,
        });

    commands
        .spawn(SpriteBundle {
            texture: textures.barrel.clone(),
            transform: Transform {
                translation: Vec3::new(50., -615., 2.),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnIngameScreen)
        .insert(Interactible {
            action: InteractibleAction::BeerBarrel,
        });

    // Set game state to Running to start systems
    ingame_state.set(IngameState::Running)
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
        if current_position.distance(target) > 10. {
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
            (bound.0 + camera_transform.translation.truncate()).extend(999.);
    }
}

fn interactibles_system(
    windows_q: Query<&Window, With<PrimaryWindow>>,
    mut camera_q: Query<
        (&Camera, &GlobalTransform, &mut MoveCameraTo),
        (With<MainCamera>, Without<Interactible>),
    >,
    mut interactibles_q: Query<(&mut Interactible, &Transform, &Handle<Image>, &mut Sprite)>,
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
        interactibles.sort_by(|a, b| b.1.translation.z.total_cmp(&a.1.translation.z));

        let mut found_collision = false;

        for (
            mut interactible,
            interactible_transform,
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
    ignored_interactibles: &mut IgnoredInteractibleActions,
) {
    match interactible.action {
        InteractibleAction::SeeBar => {
            move_camera_to.0 = Some(Vec2::new(0., -275.));
            // Deactivate SeeBar
            remove_value_from_vec(InteractibleAction::SeeBar, &mut active_interactibles.0);
            // Stop ignoring BeerBarrel
            remove_value_from_vec(InteractibleAction::BeerBarrel, &mut ignored_interactibles.0);
            // Activate ExitBar
            active_interactibles.0.push(InteractibleAction::ExitBar);
        }
        InteractibleAction::ExitBar => {
            move_camera_to.0 = Some(Vec2::new(0., 0.));
            // Deactivate ExitBar
            remove_value_from_vec(InteractibleAction::ExitBar, &mut active_interactibles.0);
            // Ignore BeerBarrel
            ignored_interactibles.0.push(InteractibleAction::BeerBarrel);
            // Activate SeeBar
            active_interactibles.0.push(InteractibleAction::SeeBar);
        }
        InteractibleAction::BeerBarrel => {}
        InteractibleAction::_None => {}
    }
}

#[derive(Component)]
struct OnPauseMenu;

#[derive(Component)]
enum PauseButtonAction {
    Resume,
    Settings,
    MainMenu(bool),
}

fn setup_pause_menu(mut commands: Commands, camera_q: Query<&Transform, With<MainCamera>>) {
    let camera_transform = camera_q.single();
    // Transparent Pause background
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: camera_transform.translation.xy().extend(111.),
                scale: Vec3::new(1920., 1080., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.95,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnPauseMenu);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexEnd,
                    ..Default::default()
                },
                // background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },
            OnPauseMenu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Tavern",
                    TextStyle {
                        font_size: 120.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                ), // .with_text_alignment(TextAlignment::Center)
                   // .with_style(Style {
                   //     // justify_self: JustifySelf::Baseline,
                   //     margin: UiRect {
                   //         // top: Val::Vh(15.),
                   //         // bottom: Val::Vh(15.),
                   //         ..Default::default()
                   //     },
                   //     ..Default::default()
                   // })
            );
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(65.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::End,
                    margin: UiRect::bottom(Val::Vh(10.)),
                    padding: UiRect::vertical(Val::Vh(10.)),
                    ..Default::default()
                },
                // background_color: BackgroundColor(Color::Rgba {
                //     red: 0.5,
                //     green: 0.5,
                //     blue: 0.5,
                //     alpha: 0.5,
                // }),
                ..Default::default()
            },
            OnPauseMenu,
        ))
        .with_children(|parent| {
            let button_style = Style {
                width: Val::Px(300.0),
                // height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(25.)),
                padding: UiRect::axes(Val::Px(15.), Val::Px(10.)),
                ..Default::default()
            };
            let button_text_style = TextStyle {
                font_size: 50.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..Default::default()
            };

            menu_button(
                parent,
                "Resume",
                PauseButtonAction::Resume,
                &button_style,
                &ButtonColors {
                    hovered: Color::rgb(0.3, 0.4, 0.4),
                    ..Default::default()
                },
                &button_text_style,
            );

            menu_button(
                parent,
                "Settings",
                PauseButtonAction::Settings,
                &button_style,
                &ButtonColors::default(),
                &button_text_style,
            );

            menu_button(
                parent,
                "Main Menu",
                PauseButtonAction::MainMenu(false),
                &button_style,
                &ButtonColors {
                    hovered: Color::rgb(0.5, 0.2, 0.2),
                    ..Default::default()
                },
                &button_text_style,
            );
        });
}

fn handle_button(
    mut ingame_state: ResMut<NextState<IngameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut ButtonColors,
            Option<&mut PauseButtonAction>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut button_colors, pause_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(mut action) = pause_button_action {
                    match *action {
                        PauseButtonAction::Resume => ingame_state.set(IngameState::Running),
                        PauseButtonAction::Settings => {}
                        PauseButtonAction::MainMenu(confirm) => {
                            if !confirm {
                                button_colors.normal = Color::rgb(0.5, 0.2, 0.2);
                                button_colors.hovered = Color::rgb(0.8, 0.2, 0.2);
                                *action = PauseButtonAction::MainMenu(true);
                            } else {
                                ingame_state.set(IngameState::ToMenu)
                            }
                        }
                    }
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn esc_to_pause(
    mut keys: ResMut<Input<KeyCode>>,
    ingame_state: Res<State<IngameState>>,
    mut ingame_next_state: ResMut<NextState<IngameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match *ingame_state.get() {
            IngameState::Running => ingame_next_state.set(IngameState::Paused),
            IngameState::Paused => ingame_next_state.set(IngameState::Running),
            _ => {}
        }
        keys.reset(KeyCode::Escape);
    }
}

fn go_to_menu(mut game_next_state: ResMut<NextState<GameState>>) {
    game_next_state.set(GameState::Menu);
}
