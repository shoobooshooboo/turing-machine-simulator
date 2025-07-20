#![allow(dead_code)]
//#![windows_subsystem = "windows"]
use bevy::prelude::*;

use crate::main_menu::PlayerIndex;

mod main_menu;


#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum GameState{
    MainMenu,
    InGame,
    Settings,
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_state(GameState::MainMenu)
    .add_systems(
        OnEnter(GameState::MainMenu),
        (main_menu::startup))
    .add_systems(
        Startup,
        (spawn_camera)
    )
    .add_systems(
        Update,
        (
            main_menu::controls,
            main_menu::button_selection.after(main_menu::controls),
        )
    )
    .insert_resource(PlayerIndex::default())
    .run();
}

fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn((Camera2d::default()));
}