#![allow(dead_code)]
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use bevy::{prelude::*, window::{WindowResized, WindowResolution}};

use crate::menus::MenuState;

mod menus;
mod games;

const BASE_WINDOW_HEIGHT: f32 = 800.0;
const BASE_WINDOW_WIDTH: f32 = 1200.0;
const BASE_WINDOW_ASPECT_RATIO: f32 = BASE_WINDOW_WIDTH / BASE_WINDOW_HEIGHT;

/// controls the current app state
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppState{
    InGame,
    InMenu,
    Paused,
    Transition,
}

#[derive(Component, Deref, DerefMut)]
pub struct BaseFontSize(f32);

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
    .add_plugins(menus::MenuPlugin)
    .add_plugins(games::GamePlugin)
    .insert_state(AppState::InMenu)
    .add_systems(
        Startup,
        spawn_camera
    )
    .add_systems(
        OnEnter(AppState::Transition),
        transition
    )
    .add_systems(
        Update,
        scale_text
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

/// scales all text in the world based on the window size
pub fn scale_text(
    mut resizes: EventReader<WindowResized>,
    mut texts: Query<(&BaseFontSize, &mut TextFont)>
){
    for event in resizes.read(){
        let height_scale = event.height / BASE_WINDOW_HEIGHT;
        let width_scale = event.width / BASE_WINDOW_WIDTH;
        let scale = height_scale.min(width_scale);
        for (base, mut actual) in &mut texts{
            actual.font_size = **base * scale;
        }
    }
}