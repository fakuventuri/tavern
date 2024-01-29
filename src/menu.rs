use crate::loading::TextureAssets;
use crate::{despawn_screen, exit_game_system, GameState};
use bevy::prelude::*;
use bevy::text::TextSettings;
use bevy::time::Stopwatch;
use bevy::window::WindowResized;

pub struct MenuPlugin;

#[derive(Component)]
struct OnMainMenuScreen;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    // Settings,
    // SettingsDisplay,
    // SettingsSound,
    #[default]
    Disabled,
    Exit,
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(TextSettings {
                allow_dynamic_font_size: true,
                ..Default::default()
            })
            .add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), (setup_menu_state, setup_camera))
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(
                Update,
                (
                    on_resize, //.run_if(in_state(GameState::Menu)),
                    handle_button.run_if(in_state(GameState::Menu)),
                    esc_to_quit.run_if(in_state(MenuState::Main)),
                    // clean_and_exit.run_if(in_state(MenuState::Exit)),
                ),
            )
            .add_systems(
                OnEnter(MenuState::Exit),
                exit_game_system.after(despawn_screen::<OnMainMenuScreen>),
            );
    }
}

fn setup_menu_state(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_camera(mut commands: Commands) {
    // Camera
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::AutoMin {
        min_width: 1920.,
        min_height: 1080.,
    };
    camera_bundle.camera_2d.clear_color =
        bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.05, 0.05, 0.05));

    commands.spawn(camera_bundle).insert(OnMainMenuScreen);
}

#[derive(Component, Clone, Copy)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.2, 0.2, 0.2),
            hovered: Color::rgb(0.35, 0.35, 0.35),
        }
    }
}

fn setup_main_menu(mut commands: Commands, textures: Res<TextureAssets>) {
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
            OnMainMenuScreen,
        ))
        .with_children(|children| {
            children.spawn(
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
            OnMainMenuScreen,
        ))
        .with_children(|children| {
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
                children,
                "Continue",
                MenuButtonAction::Continue,
                &button_style,
                &ButtonColors {
                    hovered: Color::rgb(0.4, 0.4, 0.4),
                    normal: Color::rgb(0.4, 0.4, 0.4),
                },
                &TextStyle {
                    font_size: 50.0,
                    color: Color::rgb(0.6, 0.6, 0.6),
                    ..Default::default()
                },
            );

            menu_button(
                children,
                "New Game",
                MenuButtonAction::Play,
                &button_style,
                &ButtonColors {
                    hovered: Color::rgb(0.3, 0.4, 0.4),
                    ..Default::default()
                },
                &button_text_style,
            );

            menu_button(
                children,
                "Settings",
                MenuButtonAction::Settings,
                &button_style,
                &ButtonColors::default(),
                &button_text_style,
            );

            menu_button(
                children,
                "Quit",
                MenuButtonAction::Quit(false),
                &button_style,
                &ButtonColors {
                    hovered: Color::rgb(0.5, 0.2, 0.2),
                    ..Default::default()
                },
                &button_text_style,
            );
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart, // JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Vh(8.),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(10.),
                            padding: UiRect::all(Val::VMin(2.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..Default::default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    ));
                    children.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            height: Val::VMin(6.),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });

    // Stopwatch and exit with esc msg
    commands.spawn((QuitEscTime::new(), OnMainMenuScreen));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // width: Val::Percent(40.0),
                    // height: Val::Percent(10.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::axes(Val::Px(50.), Val::Px(10.)),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.),
                    right: Val::Px(5.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.75,
                }),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Popup,
            OnMainMenuScreen,
        ))
        .with_children(|children| {
            children.spawn(TextBundle::from_section(
                "ESC again to exit game",
                TextStyle {
                    font_size: 40.,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            ));
        });
}

pub fn menu_button<T: Component>(
    children: &mut ChildBuilder<'_, '_, '_>,
    text: &str,
    action: T,
    button_style: &Style,
    button_colors: &ButtonColors,
    button_text_style: &TextStyle,
) {
    children
        .spawn((
            ButtonBundle {
                style: button_style.clone(),
                background_color: button_colors.normal.into(),
                ..Default::default()
            },
            *button_colors,
            action,
        ))
        .with_children(|children| {
            children.spawn(
                TextBundle::from_section(text, button_text_style.clone())
                    .with_text_alignment(TextAlignment::Center),
            );
        });
}

#[derive(Component)]
enum MenuButtonAction {
    Continue,
    Play,
    Settings,
    Quit(bool),
}

#[derive(Component)]
struct OpenLink(&'static str);

fn handle_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut ButtonColors,
            Option<&mut MenuButtonAction>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut button_colors, menu_button_action, open_link) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(mut action) = menu_button_action {
                    match *action {
                        MenuButtonAction::Continue => {}
                        MenuButtonAction::Play => {
                            game_state.set(GameState::Playing);
                            menu_state.set(MenuState::Disabled);
                        }
                        MenuButtonAction::Settings => {}
                        MenuButtonAction::Quit(confirm) => {
                            if !confirm {
                                button_colors.normal = Color::rgb(0.5, 0.2, 0.2);
                                button_colors.hovered = Color::rgb(0.8, 0.2, 0.2);
                                *action = MenuButtonAction::Quit(true);
                            } else {
                                menu_state.set(MenuState::Exit);
                            }
                        }
                    }
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
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

#[derive(Component)]
struct QuitEscTime {
    time: Option<Stopwatch>,
}

impl QuitEscTime {
    fn new() -> Self {
        Self { time: None }
    }
}

fn esc_to_quit(
    keys: Res<Input<KeyCode>>,
    mut quit_esc_time_query: Query<&mut QuitEscTime>,
    time: Res<Time>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut query_popup_visibility: Query<&mut Visibility, With<Popup>>,
) {
    let mut quit_esc_time = quit_esc_time_query.single_mut();
    if keys.just_pressed(KeyCode::Escape) {
        if let Some(stopw) = &quit_esc_time.time {
            if stopw.elapsed_secs() < 1. {
                menu_state.set(MenuState::Exit);
            }
        } else {
            quit_esc_time.time = Some(Stopwatch::new());
            *query_popup_visibility.single_mut() = Visibility::Visible;
        }
    }

    if let Some(stopw) = &mut quit_esc_time.time {
        if stopw.elapsed_secs() < 1. {
            stopw.tick(time.delta());
        } else {
            quit_esc_time.time = None;
            *query_popup_visibility.single_mut() = Visibility::Hidden;
        }
    }
}

#[derive(Component)]
struct Popup;

fn on_resize(mut resize_reader: EventReader<WindowResized>, mut ui_scale: ResMut<UiScale>) {
    // // Window size
    // let window = window_query.get_single().unwrap();
    // // window.resolution.set(1280., 720.);
    // let win_min = window.width().min(window.height());
    // size_factor.0 = win_min / 1080.;

    if let Some(win_resized) = resize_reader.read().last() {
        let win_min: f64 = win_resized.width.min(win_resized.height) as f64;
        ui_scale.0 = win_min / 1000.;
    }
}
