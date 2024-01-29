#![allow(clippy::type_complexity)]

// mod actions;
#[allow(dead_code, unused)]
mod audio;
mod ingame;
mod loading;
mod menu;
// #[allow(dead_code, unused)]
// mod player;

// use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::ingame::IngamePlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::{App, AppExit};
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

// Config
// WindowMode
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum ScreenMode {
    Windowed,
    BorderlessFullscreen,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_state::<GameState>()
            .insert_resource(ScreenMode::Windowed)
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                // ActionsPlugin,
                InternalAudioPlugin,
                IngamePlugin,
            ))
            .add_systems(
                Update,
                (screen_mode_with_resource, set_screen_mode_with_keys),
            );

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}

fn set_screen_mode_with_keys(
    keyboard_input: Res<Input<KeyCode>>,
    mut screen_mode: ResMut<ScreenMode>,
) {
    if keyboard_input.pressed(KeyCode::AltLeft)
        && (keyboard_input.just_pressed(KeyCode::Return) || keyboard_input.just_pressed(KeyCode::F))
    {
        match *screen_mode {
            ScreenMode::Windowed => {
                *screen_mode = ScreenMode::BorderlessFullscreen;
            }
            ScreenMode::BorderlessFullscreen => {
                *screen_mode = ScreenMode::Windowed;
            }
        }
    }
}

fn screen_mode_with_resource(screen_mode: Res<ScreenMode>, mut windows: Query<&mut Window>) {
    if screen_mode.is_changed() {
        let mut window = windows.single_mut();
        match *screen_mode {
            ScreenMode::BorderlessFullscreen => {
                window.mode = bevy::window::WindowMode::BorderlessFullscreen
            }
            ScreenMode::Windowed => window.mode = bevy::window::WindowMode::Windowed,
        }
    }
}

pub fn exit_game_system(mut app_exit_events: EventWriter<AppExit>) {
    // Exit event
    app_exit_events.send(AppExit);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn remove_value_from_vec<T: PartialEq>(value_to_remove: T, vec: &mut Vec<T>) {
    vec.swap_remove(
        vec.iter()
            .position(|x| *x == value_to_remove)
            .expect("InteractibleAction to remove is not active."),
    );
}
