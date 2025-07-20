#![allow(dead_code)]
//#![windows_subsystem = "windows"]
use bevy::{prelude::*, window::{WindowResolution}};

mod menus;

use menus::*;

const BASE_WINDOW_HEIGHT: f32 = 800.0;
const BASE_WINDOW_WIDTH: f32 = 1200.0;
const BASE_WINDOW_ASPECT_RATIO: f32 = BASE_WINDOW_WIDTH / BASE_WINDOW_HEIGHT;


#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState{
    MainMenu,
    PlayGameMenu,
    SettingsMenu,
    CreditsMenu,
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
    .insert_state(GameState::MainMenu)
    .add_systems(
        OnEnter(GameState::MainMenu),
        main_menu::startup)
    .add_systems(
        Startup,
        spawn_camera
    )
    .add_systems(
        OnExit(GameState::MainMenu),
         main_menu::exit)
    .add_systems(
    Update,
    (
        main_menu::controls.run_if(in_state(GameState::MainMenu)),
        main_menu::button_selection.run_if(in_state(GameState::MainMenu)).after(main_menu::controls),
    ))
    .add_systems(
        Update,
        menus::scale_text
    )
    .insert_resource(PlayerIndex::default())
    .run();
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2d::default());
}