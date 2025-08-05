#![allow(dead_code)]
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use std::{fs, path::Path};

use bevy::{audio::Volume, prelude::*, window::{WindowResized, WindowResolution}};

use crate::{menus::MenuState, post_processing::{PostProcessSettings, TimeData}};

mod menus;
mod games;
mod pause;
mod post_processing;

const BASE_WINDOW_HEIGHT: f32 = 800.0;
const BASE_WINDOW_WIDTH: f32 = 1200.0;
const BASE_WINDOW_ASPECT_RATIO: f32 = BASE_WINDOW_WIDTH / BASE_WINDOW_HEIGHT;

//all these are based on vibes
const DEFAULT_CHROMATIC_ABERATION: f32 = 0.0021;
const MAX_CHROMATIC_ABERATION: f32 = 0.01224853;
const MIN_CHROMATIC_ABERATION: f32 = 0.0;

const AUDIO_FILE_PREFIX: &'static str = "audio\\";
const FONT_FILE: &'static str = "dos_font.ttf";
const SETTINGS_FILE: &'static str = "assets\\saves\\settings";
const SHADER_FILE: &'static str = "shaders\\post_processing.wgsl";

/// controls the current app state
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AppState{
    InGame,
    InMenu,
    Paused,
    Transition,
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum UpdateSet{
    Input,
    Logic,
    UI,
    Misc,
}

#[derive(Component, Deref, DerefMut)]
pub struct BaseFontSize(f32);

#[derive(Resource, Deref, DerefMut)]
pub struct VolumeSetting(Volume);

#[derive(Resource, Deref, DerefMut)]
pub struct DefaultFont(Handle<Font>);

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
    .add_plugins(pause::PausePlugin)
    .add_plugins(post_processing::PostProcessPlugin)
    .insert_state(AppState::Transition)
    .configure_sets(
        Update,
        (
            (
            UpdateSet::Input,
            UpdateSet::Logic,
            UpdateSet::Misc,
            UpdateSet::UI,
            ).chain(),
        )
    )
    .add_systems(
        Startup,
        setup
    )
    .add_systems(
        Update,
        (
            scale_text.in_set(UpdateSet::UI),
            save_setting.in_set(UpdateSet::Misc),
        )
    )
    .add_systems(
        OnEnter(AppState::Transition),
        transition
    )
    .run();
}

///spawns camera and loads default font and settings
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let font = asset_server.load(FONT_FILE);
    commands.insert_resource(DefaultFont(font));

    commands.insert_resource(VolumeSetting(Volume::Linear(1.0)));

    let mut chromatic_abberation = DEFAULT_CHROMATIC_ABERATION;
    let contents = fs::read_to_string(Path::new(SETTINGS_FILE)).unwrap_or_default();
    for line in contents.lines(){
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        let identifier = match line.get(0){
            Some(s) => s.to_lowercase(),
            None => continue,
        };
        let value: f32 = match line.get(1){
            Some(s) => match s.parse(){
                    Ok(v) => v,
                    Err(_) => continue,
                },
            None => continue,
        };

        match identifier.as_str(){
            "volume" => commands.insert_resource(VolumeSetting(Volume::Linear(value))),
            "chromab" => chromatic_abberation = value,
            _ => (),
        }
    }

    commands.spawn((
        Camera2d::default(),
        PostProcessSettings {
            intensity: chromatic_abberation.clamp(MIN_CHROMATIC_ABERATION, MAX_CHROMATIC_ABERATION),
        },
        TimeData {time: 0.0},
    ));
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

pub fn save_setting(
    exit: EventReader<AppExit>,
    volume: Res<VolumeSetting>,
    chromab_query: Query<&PostProcessSettings>,
){
    if exit.is_empty(){ return; }
    let chromab = chromab_query.single().unwrap().intensity;
    let contents = "volume ".to_string() + &volume.0.to_linear().to_string()
                         + "\nchromab " + &chromab.to_string();

    let _ = fs::write(SETTINGS_FILE, contents);
}