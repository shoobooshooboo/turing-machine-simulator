use bevy::{audio::PlaybackMode, prelude::*};
use crate::{games::{GameState, SaveFileIndex}, AppState, CurVolume, DefaultFont, AUDIO_FILE_PREFIX};
use std::collections::HashMap;
use std::slice::Iter;

const BUTTON_UNSELECTED_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const BUTTON_SELECTED_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BUTTON_OUTLINE_UNSELECTED_WIDTH_PER: f32 = 0.5;
const BUTTON_OUTLINE_SELECTED_WIDTH_PER: f32 = 0.75;

const AUDIO_FILES: [&'static str; 3] = ["menu-move.mp3", "menu-select.mp3", "menu-back.mp3"];
///types of menu sound effects.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum MenuSoundType{
    Move,
    Select,
    Back,
}

impl MenuSoundType{
    pub fn iterator() -> Iter<'static, MenuSoundType>{
        static MENU_SOUND_TYPES: [MenuSoundType; AUDIO_FILES.len()] = [MenuSoundType::Move, MenuSoundType::Select, MenuSoundType::Back];
        MENU_SOUND_TYPES.iter()
    }
}

///marker for ui objects of the menu
#[derive(Component)]
struct MenuUI;

///index of a button
#[derive(Component, Deref, DerefMut)]
struct ButtonIndex(usize);

#[derive(Resource, Deref)]
struct MenuSounds(HashMap<MenuSoundType, Handle<AudioSource>>);

///player's current selected button
#[derive(Resource, Deref, DerefMut, Default)]
struct PlayerIndex(usize);

///total number of buttons in current menu
#[derive(Resource, Deref, DerefMut, Default)]
pub struct ButtonCount(usize);

pub enum TransitionType{
    In,
    Out,
}

#[derive(Event, Deref)]
struct PlayMenuSoundEvent(MenuSoundType);

mod main_menu;
mod settings_menu;
mod credits_menu;
mod game_menu;
mod sandbox_menu;

/// controls the current menu
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenuState{
    MainMenu,
    GameMenu,
    SandboxMenu,
    SettingsMenu,
    CreditsMenu,
    QuitMenu,
    None,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App){
    app
    .insert_state(MenuState::MainMenu)
    .insert_resource(PlayerIndex::default())
    .insert_resource(ButtonCount::default())
    .add_event::<PlayMenuSoundEvent>()
    .add_systems(
        Startup,
        load_audio,
    )
    .add_systems(
        OnEnter(AppState::InMenu),
        load_ui
    )
    .add_systems(
        OnTransition{exited: AppState::InMenu, entered: AppState::Transition},
         unload_ui
    )
    .add_systems(
    Update,
    (
        (
        controls,
        button_selection,
        settings_menu::update_sliders.run_if(in_state(MenuState::SettingsMenu)),
        settings_menu::slider_controls.run_if(in_state(MenuState::SettingsMenu)),
        ).chain(),
        play_menu_move_sound,
    ).run_if(in_state(AppState::InMenu)));
    }
}

fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let mut sounds = HashMap::new();
    for (&menu_sound_type, &file_name) in MenuSoundType::iterator().zip(AUDIO_FILES.iter()){
        let path = AUDIO_FILE_PREFIX.to_owned() + file_name;
        sounds.insert(menu_sound_type, asset_server.load(path));
    }

    commands.insert_resource(MenuSounds(sounds));
}

/// sets a button's background color and border width depending off it is selected or not
fn button_selection(
    player_index: Res<PlayerIndex>,
    mut buttons: Query<(&ButtonIndex, &mut BackgroundColor, &mut Outline), With<Button>>,
){
    for (index, mut bgc, mut outline) in &mut buttons{
        if **index == **player_index{
            bgc.0 = BUTTON_SELECTED_COLOR;
            outline.width = Val::Percent(BUTTON_OUTLINE_SELECTED_WIDTH_PER);
        }
        else{
            outline.width = Val::Percent(BUTTON_OUTLINE_UNSELECTED_WIDTH_PER);
            bgc.0 = BUTTON_UNSELECTED_COLOR;
        }
    }
}

/// handles controls while in the menu
fn controls(
    mut player_index: ResMut<PlayerIndex>,
    save_file_index: ResMut<SaveFileIndex>, 
    inputs: Res<ButtonInput<KeyCode>>,
    exit: EventWriter<AppExit>,
    menu_state: Res<State<MenuState>>,
    next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,    
    next_game_state: ResMut<NextState<GameState>>,    
    button_count: Res<ButtonCount>,
    mut sound_player: EventWriter<PlayMenuSoundEvent>,
){
    if inputs.just_pressed(KeyCode::ArrowUp){
        **player_index = player_index.checked_sub(1).unwrap_or(**button_count - 1);
        sound_player.write(PlayMenuSoundEvent(MenuSoundType::Move));
    }else if inputs.just_pressed(KeyCode::ArrowDown){
        **player_index = (**player_index + 1) % **button_count;
        sound_player.write(PlayMenuSoundEvent(MenuSoundType::Move));
    }
    **player_index = player_index.clamp(0, **button_count - 1);

    if inputs.just_pressed(KeyCode::Enter){
        next_app_state.set(AppState::Transition);
        match match **menu_state{
            MenuState::MainMenu => main_menu::transition(player_index, exit, next_menu_state),
            MenuState::GameMenu => game_menu::transition(player_index, next_menu_state, next_game_state),
            MenuState::CreditsMenu => credits_menu::transition(next_menu_state),
            MenuState::SandboxMenu => sandbox_menu::transition(player_index, save_file_index, next_menu_state, next_game_state),
            MenuState::SettingsMenu => settings_menu::transition(player_index, next_menu_state),
            _ => panic!("unimplemented menu"),
        }{
            TransitionType::In => sound_player.write(PlayMenuSoundEvent(MenuSoundType::Select)),
            TransitionType::Out => sound_player.write(PlayMenuSoundEvent(MenuSoundType::Back)),
        };
    }else if inputs.just_pressed(KeyCode::Escape){
        next_app_state.set(AppState::Transition);
        sound_player.write(PlayMenuSoundEvent(MenuSoundType::Back));
        match **menu_state{
            MenuState::MainMenu => main_menu::detransition(exit),
            MenuState::GameMenu => game_menu::detransition(next_menu_state),
            MenuState::CreditsMenu => credits_menu::detransition(next_menu_state),
            MenuState::SandboxMenu => sandbox_menu::detransition(next_menu_state),
            MenuState::SettingsMenu => settings_menu::detransition(next_menu_state),
            _ => panic!("unimplemented menu"),
        }
    }
}

/// unloads all menu ui elements
fn unload_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<MenuUI>>,
){
    for entity in ui_elements{
        commands.entity(entity).despawn();
    }
}

/// loads all menu ui elements
fn load_ui(
    commands: Commands,
    button_count: ResMut<ButtonCount>,
    menu_state: Res<State<MenuState>>,
    mut player_index: ResMut<PlayerIndex>,
    meshes: ResMut<Assets<Mesh>>,
    mats: ResMut<Assets<ColorMaterial>>,
    volume: Res<CurVolume>,
    font: Res<DefaultFont>,
){
    **player_index = 0;
    match **menu_state{
        MenuState::MainMenu => main_menu::load(commands, button_count, font),
        MenuState::GameMenu => game_menu::load(commands, button_count, font), 
        MenuState::CreditsMenu => credits_menu::load(commands, button_count, font),
        MenuState::SandboxMenu => sandbox_menu::load(commands, button_count, font),
        MenuState::SettingsMenu => settings_menu::load(commands, button_count, meshes, mats, volume, font),
        _ => print!("unimplemented menu"),
    }
}

fn play_menu_move_sound(
    mut sound_events: EventReader<PlayMenuSoundEvent>,
    mut commands: Commands,
    volume: Res<CurVolume>,
    sounds: Res<MenuSounds>,
){
    for sound_type in sound_events.read(){
        commands.spawn((
            AudioPlayer(sounds[sound_type].clone()), 
            PlaybackSettings{
                mode: PlaybackMode::Despawn,
                volume: **volume, 
                ..Default::default()
            }
        ));
    }
}