use bevy::{prelude::*, render::mesh::Triangle2dMeshBuilder, text::FontSmoothing,};
use crate::{BaseFontSize, AppState, GameState};

const CELL_SPACING_PER: f32 = 5.0;
const CELL_COUNT: usize = 1_000;
const VISIBLE_CELL_COUNT: i8 = 5;
const CELL_WIDTH: f32 = (100.0 - (CELL_SPACING_PER * (VISIBLE_CELL_COUNT + 1) as f32)) / VISIBLE_CELL_COUNT as f32;
const BORDER_WIDTH_PER: f32 = 3.0;
const MAIN_CELL_BORDER_WIDTH_PER: f32 = 5.0;
const DEFAULT_CELL_CHAR: char = '_';
const TEXT_FONT_SIZE: f32 = 80.0;

mod sandbox;

#[derive(Component)]
struct GameUI;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
struct Cell(i32);

#[derive(Resource, Deref, DerefMut)]
struct Tape{
    cells: Box<[char; CELL_COUNT]>
}

impl Default for Tape{
    fn default() -> Self {
        Self{
            cells: Box::new([DEFAULT_CELL_CHAR; CELL_COUNT])
        }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct CursorIndex(usize);

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_state(GameState::None)
        .insert_resource(Tape::default())
        .insert_resource(CursorIndex::default())
        .add_systems(
        OnEnter(AppState::InGame),
        load_game
        )
        .add_systems(
            Update,
            (
                controls.run_if(in_state(AppState::InGame)),
                update_cells.run_if(in_state(AppState::InGame)),
        ).chain())
        ;
    }
}

/// loads the game elements
fn load_game(
    mut commands: Commands,
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
            Outline::new(if i == 2 {Val::Percent(MAIN_CELL_BORDER_WIDTH_PER)} else {Val::Percent(BORDER_WIDTH_PER)},
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
        GameState::Sandbox => sandbox::load(commands, tape),
        _ => println!("unimplemented menu"),
    }
}

///handles user inputs
fn controls(
    mut cursor: ResMut<CursorIndex>,
    inputs: Res<ButtonInput<KeyCode>>,
    mut cells: Query<&mut Cell>,
    mut tape: ResMut<Tape>,
){
    let initial_cursor = **cursor;
    if inputs.just_pressed(KeyCode::ArrowLeft){
        **cursor = cursor.checked_sub(1).unwrap_or(0);
    }
    if inputs.just_pressed(KeyCode::ArrowRight){
        **cursor += 1;
    }

    **cursor = cursor.clamp(0, CELL_COUNT - 1);

    let cursor_moved = initial_cursor != **cursor;
    if cursor_moved{
        for mut cell_index in &mut cells{
            **cell_index += if initial_cursor < **cursor {1} else {-1};
        }
    }
    
    if inputs.just_pressed(KeyCode::Backspace){
        tape[**cursor] = DEFAULT_CELL_CHAR;
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
fn unload_game(){}