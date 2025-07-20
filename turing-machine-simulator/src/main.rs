#![allow(dead_code)]
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use bevy::{prelude::*, window::{WindowResolution}};

mod menus;

use menus::*;

const BASE_WINDOW_HEIGHT: f32 = 800.0;
const BASE_WINDOW_WIDTH: f32 = 1200.0;
const BASE_WINDOW_ASPECT_RATIO: f32 = BASE_WINDOW_WIDTH / BASE_WINDOW_HEIGHT;


#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MenuState{
    MainMenu,
    PlayGameMenu,
    SettingsMenu,
    CreditsMenu,
    QuitMenu,
}

#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState{
    InGame,
    InMenu,
    Paused,
    Transition,
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
    .insert_state(GameState::InMenu)
    .insert_resource(PlayerIndex::default())
    .insert_resource(ButtonCount::default())
    .add_systems(
        Startup,
        spawn_camera
    )
    .add_systems(
        OnEnter(GameState::InMenu),
        menus::startup)
    .add_systems(
        OnExit(GameState::InMenu),
         menus::unload_ui)
    .add_systems(
    Update,
    (
        menus::controls.run_if(in_state(GameState::InMenu)),
        menus::button_selection.run_if(in_state(GameState::InMenu)).after(menus::controls),
    ))
    .add_systems(
        Update,
        menus::scale_text
    )
    .run();
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2d::default());
}