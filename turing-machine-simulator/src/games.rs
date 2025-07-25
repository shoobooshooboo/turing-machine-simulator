use std::{collections::HashMap, fs, path::Path, slice::Iter};
use bevy::{audio::PlaybackMode, input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*, render::mesh::Triangle2dMeshBuilder, text::FontSmoothing};
use crate::{menus::MenuState, AppState, BaseFontSize, CurVolume, AUDIO_FILE_PREFIX};

//Tape Cells
const CELL_COUNT: usize = 1_000_000;
const DEFAULT_CELL_CHAR: char = '_';

//Visual Cells
const CELL_SPACING_PER: f32 = 5.0;
const VISIBLE_CELL_COUNT: i8 = 5;
const CELL_WIDTH: f32 = (100.0 - (CELL_SPACING_PER * (VISIBLE_CELL_COUNT + 1) as f32)) / VISIBLE_CELL_COUNT as f32;
const BORDER_WIDTH_PER: f32 = 3.0;
const MAIN_CELL_BORDER_WIDTH_PER: f32 = 5.0;
const TEXT_FONT_SIZE: f32 = 80.0;

const SAVE_FILE_PATH: &'static str = "assets/saves/world";
const AUDIO_FILES: [&'static str; 5] = ["game-move.mp3", "game-cant-move.mp3", "game-select.mp3", "game-write.mp3", "game-delete.mp3"];

///types of in-game sound effects.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GameSoundType{
    Move,
    CantMove,
    Select,
    Write,
    Delete,
}

impl GameSoundType{
    pub fn iterator() -> Iter<'static, GameSoundType>{
        static GAME_SOUND_TYPES: [GameSoundType; AUDIO_FILES.len()] = [GameSoundType::Move, GameSoundType::CantMove, GameSoundType::Select, GameSoundType::Write, GameSoundType::Delete];
        GAME_SOUND_TYPES.iter()
    }
}

#[derive(Resource, Deref)]
struct GameSounds(HashMap<GameSoundType, Handle<AudioSource>>);

#[derive(Component)]
struct GameUI;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
struct Cell(i32);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SaveFileIndex(Option<usize>);

#[derive(Resource, Deref, DerefMut)]
struct Tape{
    cells: Box<Vec<char>>
}

impl Default for Tape{
    fn default() -> Self {
        let mut cells = Box::new(Vec::new());
        cells.resize(CELL_COUNT, DEFAULT_CELL_CHAR);
        Self{
            cells
        }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct CursorIndex(usize);

/// controls the current gamemode
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState{
    Sandbox,
    None,
}

mod sandbox;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_state(GameState::None)
        .insert_resource(Tape::default())
        .insert_resource(CursorIndex::default())
        .insert_resource(SaveFileIndex::default())
        .add_systems(
            Startup,
            load_sounds,
        )
        .add_systems(
        OnEnter(AppState::InGame),
        load_ui
        )
        .add_systems(
            Update,
            (
                controls.run_if(in_state(AppState::InGame)),
                write_to_cell.run_if(in_state(AppState::InGame)),
                update_cells.run_if(in_state(AppState::InGame)),
        ).chain())
        .add_systems(
            OnExit(AppState::InGame),
            unload_ui
        )
        ;
    }
}

fn load_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let mut sounds = HashMap::new();
    for (&sound_type, &file_name) in GameSoundType::iterator().zip(AUDIO_FILES.iter()){
        let path = AUDIO_FILE_PREFIX.to_owned() + file_name;
        sounds.insert(sound_type, asset_server.load(path));
    }
    commands.insert_resource(GameSounds(sounds));
}

/// loads the game elements
fn load_ui(
    mut commands: Commands,
    save_file_index: Res<SaveFileIndex>,
    tape: ResMut<Tape>,
    game_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    mut cursor_index: ResMut<CursorIndex>,
){
    //reset cursor
    **cursor_index = 0;

    //loads the cells
    for i in 0..VISIBLE_CELL_COUNT{
        commands.spawn((
            GameUI,
            Cell((i - VISIBLE_CELL_COUNT / 2) as i32),
            Node{
                position_type: PositionType::Absolute,
                top: Val::Percent(CELL_SPACING_PER),
                left: Val::Percent(CELL_SPACING_PER * (i + 1) as f32 + CELL_WIDTH * i as f32),
                height: Val::Vw(CELL_WIDTH),
                width: Val::Vw(CELL_WIDTH),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            Visibility::Visible,
            BackgroundColor(Color::NONE),
            Outline::new(if i == VISIBLE_CELL_COUNT / 2 {Val::Percent(MAIN_CELL_BORDER_WIDTH_PER)} else {Val::Percent(BORDER_WIDTH_PER)},
             Val::Px(0.0), Color::BLACK),
        )).with_child((
            Text::new("_"),
            TextFont{
                font_size: TEXT_FONT_SIZE,
                font_smoothing: FontSmoothing::AntiAliased,
                ..Default::default()
            },
            BaseFontSize(TEXT_FONT_SIZE),
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    }

    //loads cursor
    commands.spawn((
        GameUI,
        Mesh2d(meshes.add(
            Triangle2dMeshBuilder::new(Vec2::new(0.0, 100.0),
            Vec2::new(-50.0, 0.0),
            Vec2::new(50.0, 0.0),
        ))),
        MeshMaterial2d(mats.add(Color::BLACK)),
        //Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
    
    match **game_state{
        GameState::Sandbox => sandbox::load(commands, save_file_index, tape),
        _ => println!("unimplemented menu"),
    }
}

///handles user inputs
fn controls(
    mut cursor: ResMut<CursorIndex>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut cells: Query<&mut Cell>,
    mut tape: ResMut<Tape>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut commands: Commands,
    sounds: Res<GameSounds>,
    volume: Res<CurVolume>,
){
    let volume = volume.0;
    let initial_cursor = **cursor;
    let mut cursor_tried_move = false; 
    if inputs.just_pressed(KeyCode::ArrowLeft){
        **cursor = cursor.checked_sub(1).unwrap_or(0);
        cursor_tried_move = true;
    }
    if inputs.just_pressed(KeyCode::ArrowRight){
        **cursor += 1;
        cursor_tried_move = true;
    }

    **cursor = cursor.clamp(0, CELL_COUNT - 1);

    let cursor_moved = initial_cursor != **cursor;
    if cursor_moved{
        for mut cell_index in &mut cells{
            **cell_index += if initial_cursor < **cursor {1} else {-1};
        }
        commands.spawn((AudioPlayer::new(sounds[&GameSoundType::Move].clone()), PlaybackSettings{mode: PlaybackMode::Despawn, volume, ..Default::default()}));
    }else if cursor_tried_move{
        commands.spawn((AudioPlayer::new(sounds[&GameSoundType::CantMove].clone()), PlaybackSettings{mode: PlaybackMode::Despawn, volume, ..Default::default()}));
    }
    
    if inputs.just_pressed(KeyCode::Backspace){
        tape[**cursor] = DEFAULT_CELL_CHAR;
        commands.spawn((AudioPlayer::new(sounds[&GameSoundType::Delete].clone()), PlaybackSettings{mode: PlaybackMode::Despawn, volume, ..Default::default()}));
    }

    if inputs.just_pressed(KeyCode::Escape){
        next_game_state.set(GameState::None);
        next_app_state.set(AppState::Transition);
        next_menu_state.set(MenuState::GameMenu);
    }
}

fn write_to_cell(
    cursor: Res<CursorIndex>,
    mut tape: ResMut<Tape>,
    mut keyboard: EventReader<KeyboardInput>,
    mut commands: Commands,
    sounds: Res<GameSounds>,
    volume: Res<CurVolume>,
){
    let volume = volume.0;
    if keyboard.is_empty(){
        return;
    }
    let mut char_to_write = None;
    for e in keyboard.read(){
        if e.state == ButtonState::Released{
            continue;
        }

        match &e.logical_key{
            Key::Space => char_to_write = Some(' '),
            Key::Character(c) => char_to_write = Some(c.chars().next().unwrap()),
            _ => (),
        }
    }

    if let Some(c) = char_to_write{
        tape[**cursor] = c;
        commands.spawn((AudioPlayer::new(sounds[&GameSoundType::Write].clone()), PlaybackSettings{mode: PlaybackMode::Despawn, volume, ..Default::default()}));
    }
}

fn update_cells(
    tape: Res<Tape>,
    mut cells: Query<(&Cell, &mut Children, &mut Visibility)>,
    mut children_query: Query<&mut Text>,
){
    for (&cell_index, children, mut vis) in &mut cells{
        match tape.get(*cell_index as usize){
            None => {
                *vis = Visibility::Hidden;
            },
            Some(&c) => {
                *vis = Visibility::Visible;
                let child = children.iter().next().unwrap();
                if let Ok(mut text) = children_query.get_mut(child){
                    text.0 = c.to_string();
                }
            }
        }
    }
}

///unloads all game elements
fn unload_ui(
    mut commands: Commands,
    mut save_file_index: ResMut<SaveFileIndex>,
    mut ui_elements: Query<Entity, With<GameUI>>,
    mut tape: ResMut<Tape>,
){
    for entity in &mut ui_elements{
        commands.get_entity(entity).unwrap().despawn();
    }

    match **save_file_index{
        None => (),
        Some(i) =>{
            let mut contents = Box::new(String::new());
            let mut empty_count = 0;
            for &cell in tape.iter(){
                if cell == '_'{
                    empty_count += 1;
                }else{
                    contents.push_str(&"_".repeat(empty_count));
                    empty_count = 0;
                    contents.push(cell);
                }
            }

            let _ = fs::write(Path::new(&format!("{}{}.sav", SAVE_FILE_PATH, i)), *contents);
            **save_file_index = None;
        } 
    }

    for cell in tape.iter_mut(){
        *cell = DEFAULT_CELL_CHAR;
    }
}