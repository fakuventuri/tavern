use bevy::prelude::*;

use crate::{
    menu::{menu_button, settings::{OnSettingsMenuScreen, SelectedOption}, ButtonColors}, ScreenMode, TEXT_COLOR
};

use super::{IngameState, MainCameraIngame};

#[derive(Component)]
pub struct OnPauseMenu;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Settings,
    BackToPaused,
    MainMenu(bool),
}

pub fn setup_pause_menu(
    mut commands: Commands,
    camera_q: Query<&Transform, With<MainCameraIngame>>,
) {
    // Transparent Pause background
    let camera_transform = camera_q.single();
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
        .with_children(|child_builder| {
            child_builder.spawn(
                TextBundle::from_section(
                    "Tavern",
                    TextStyle {
                        font_size: 120.,
                        color: TEXT_COLOR,
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
        .with_children(|child_builder| {
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
                color: TEXT_COLOR,
                ..Default::default()
            };

            menu_button(
                child_builder,
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
                child_builder,
                "Settings",
                PauseButtonAction::Settings,
                &button_style,
                &ButtonColors::default(),
                &button_text_style,
            );

            menu_button(
                child_builder,
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn settings_pause_setup(
    mut commands: Commands,
    screen_mode: Res<ScreenMode>,
    camera_q: Query<&Transform, (With<Camera>, With<MainCameraIngame>)>,
) {
    // Transparent Pause background
    let camera_transform = camera_q.single();
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
                    alpha: 0.98,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OnSettingsMenuScreen);

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
            OnSettingsMenuScreen,
        ))
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 120.,
                    color: TEXT_COLOR,
                    ..Default::default()
                },
            ));
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
            OnSettingsMenuScreen,
        ))
        .with_children(|child_builder| {
            let button_style = Style {
                // width: Val::Px(300.0),
                // height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(25.)),
                padding: UiRect::axes(Val::Px(15.), Val::Px(10.)),
                ..Default::default()
            };
            let button_text_style = TextStyle {
                font_size: 50.0,
                color: TEXT_COLOR,
                ..Default::default()
            };

            child_builder
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(15.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|child_builder| {
                    // Display a label for the current setting
                    child_builder.spawn(
                        TextBundle::from_section("Window Mode:", button_text_style.clone())
                            .with_text_alignment(TextAlignment::Center)
                            .with_style(Style {
                                // width: Val::Px(300.0),
                                // height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect {
                                    right: Val::Px(35.),
                                    bottom: Val::Px(25.),
                                    ..Default::default()
                                },
                                // padding: UiRect::axes(Val::Px(15.), Val::Px(10.)),
                                ..Default::default()
                            }),
                    );
                    // Display a button for each possible value
                    for screen_mode_setting in
                        [ScreenMode::BorderlessFullscreen, ScreenMode::Windowed]
                    {
                        let mut entity = child_builder.spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            screen_mode_setting,
                        ));
                        entity.with_children(|child_builder| {
                            child_builder.spawn(TextBundle::from_section(
                                format!("{screen_mode_setting:?}"),
                                button_text_style.clone(),
                            ));
                        });
                        if *screen_mode == screen_mode_setting {
                            entity.insert(SelectedOption);
                        }
                    }
                });

            menu_button(
                child_builder,
                "Back",
                PauseButtonAction::BackToPaused,
                &Style {
                    // width: Val::Px(300.0),
                    // height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(50.)),
                    padding: UiRect::axes(Val::Px(15.), Val::Px(10.)),
                    ..Default::default()
                },
                &ButtonColors::default(),
                &button_text_style,
            );
        });
}

pub fn handle_button(
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
                        PauseButtonAction::Settings => ingame_state.set(IngameState::Settings),
                        PauseButtonAction::BackToPaused => ingame_state.set(IngameState::Paused),
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
