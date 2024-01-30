use bevy::prelude::*;

use crate::{ScreenMode, TEXT_COLOR};

use super::{menu_button, ButtonColors, MenuButtonAction, MenuState};

#[derive(Component)]
pub struct OnSettingsMenuScreen;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn settings_main_menu_setup(mut commands: Commands, screen_mode: Res<ScreenMode>) {
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
                MenuButtonAction::BackToMainMenu,
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

// This system handles changing all buttons color based on mouse interaction
pub fn settings_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (With<Interaction>, (With<Button>, With<ScreenMode>)),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

pub fn setting_button_handle<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (With<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_color) = selected_query.single_mut();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        } else if *setting == *button_setting {
            if previous_button != entity {
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
            }
        }
    }
}

pub fn esc_back_to_main_menu(
    mut keys: ResMut<Input<KeyCode>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.reset(KeyCode::Escape);
        menu_state.set(MenuState::Main);
    }
}
