#![allow(dead_code)]
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use bevy::{prelude::*, window::{WindowResolution}};

mod menus;
mod games;

use menus::*;

use crate::games::Tape;

const BASE_WINDOW_HEIGHT: f32 = 800.0;
const BASE_WINDOW_WIDTH: f32 = 1200.0;
const BASE_WINDOW_ASPECT_RATIO: f32 = BASE_WINDOW_WIDTH / BASE_WINDOW_HEIGHT;

/// controls the current menu
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MenuState{
    MainMenu,
    GameMenu,
    SettingsMenu,
    CreditsMenu,
    QuitMenu,
    None,
}

/// controls the current app state
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppState{
    InGame,
    InMenu,
    Paused,
    Transition,
}

/// controls the current gamemode
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState{
    Sandbox,
    None,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window{
            title: "Turing Machine Simulator!".to_string(),
            resolution: WindowResolution::new(BASE_WINDOW_WIDTH, BASE_WINDOW_HEIGHT),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..Default::default()
        }),
        ..Default::default()
    }))
    .insert_state(MenuState::MainMenu)
    .insert_state(AppState::InMenu)
    .insert_state(GameState::None)
    .insert_resource(PlayerIndex::default())
    .insert_resource(ButtonCount::default())
    .insert_resource(Tape::default())
    .add_systems(
        Startup,
        spawn_camera
    )
    .add_systems(
        Update,
        menus::scale_text
    )
    .add_systems(
        OnEnter(AppState::Transition),
        transition
    )
    .add_systems(
        OnEnter(AppState::InMenu),
        menus::load_ui
    )
    .add_systems(
        OnTransition{exited: AppState::InMenu, entered: AppState::Transition},
         menus::unload_ui
    )
    .add_systems(
    Update,
    (
        menus::controls.run_if(in_state(AppState::InMenu)),
        menus::button_selection.run_if(in_state(AppState::InMenu)).after(menus::controls),
    ))
    .add_systems(
        OnEnter(AppState::InGame),
        games::load
    )
    .run();
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2d::default());
}

fn transition(
    menu_state: Res<State<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>, 
){
    match **menu_state{
        MenuState::None => next_app_state.set(AppState::InGame),
        _ => next_app_state.set(AppState::InMenu),
    }
}