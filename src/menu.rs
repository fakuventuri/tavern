use crate::loading::TextureAssets;
use crate::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::text::TextSettings;
use bevy::window::WindowResized;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(TextSettings {
                allow_dynamic_font_size: true,
                ..default()
            })
            .add_systems(OnEnter(GameState::Menu), (setup_camera, setup_menu))
            .add_systems(
                Update,
                (
                    on_resize, //.run_if(in_state(GameState::Menu)),
                    handle_button.run_if(in_state(GameState::Menu)),
                ),
            )
            .add_systems(OnExit(GameState::Menu), despawn_screen::<OnMenuScreen>);
    }
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1080.0);
    // commands.spawn(camera_bundle);
}

#[derive(Component, Clone, Copy)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.2, 0.2, 0.2),
            hovered: Color::rgb(0.3, 0.3, 0.3),
        }
    }
}

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

#[derive(Component)]
struct OnMenuScreen;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                // background_color: BackgroundColor(Color::RED),
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|children| {
            children.spawn(
                TextBundle::from_section(
                    "Tavern",
                    TextStyle {
                        font_size: 120.,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
                // .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    // justify_self: JustifySelf::Baseline,
                    margin: UiRect {
                        // top: Val::Vh(15.),
                        // bottom: Val::Vh(15.),
                        ..Default::default()
                    },
                    ..default()
                }),
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
            OnMenuScreen,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            let button_style = Style {
                // width: Val::Px(140.0),
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
                ..default()
            };

            children
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section("Play", button_text_style.clone()));
                });

            children
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section("Play", button_text_style.clone()));
                });

            children
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section("Play", button_text_style.clone()));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
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
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 25.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    children.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            height: Val::VMin(6.),
                            ..default()
                        },
                        ..default()
                    });
                });

            // children
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Auto,
            //                 height: Val::Vh(8.),
            //                 justify_content: JustifyContent::SpaceAround,
            //                 align_items: AlignItems::Center,
            //                 column_gap: Val::Px(10.),
            //                 padding: UiRect::all(Val::VMin(2.)),
            //                 ..default()
            //             },
            //             background_color: Color::NONE.into(),
            //             ..Default::default()
            //         },
            //         ButtonColors {
            //             normal: Color::NONE,
            //             hovered: Color::rgb(0.25, 0.25, 0.25),
            //         },
            //         OpenLink("https://github.com/NiklasEi/bevy_game_template"),
            //     ))
            //     .with_children(|children| {
            //         children.spawn(TextBundle::from_section(
            //             "Open source",
            //             TextStyle {
            //                 font_size: 25.0,
            //                 color: Color::rgb(0.9, 0.9, 0.9),
            //                 ..default()
            //             },
            //         ));
            //         children.spawn(ImageBundle {
            //             image: textures.github.clone().into(),
            //             style: Style {
            //                 height: Val::VMin(6.),
            //                 ..default()
            //             },
            //             ..default()
            //         });
            //     });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

fn handle_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    // mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
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
