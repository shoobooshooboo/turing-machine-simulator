use bevy::{prelude::*, render::mesh::Triangle2dMeshBuilder, text::FontSmoothing,};
use crate::{BaseFontSize, AppState, GameState};

const CELL_SPACING_PER: f32 = 5.0;
const VISIBLE_CELL_COUNT: u8 = 5;
const CELL_WIDTH: f32 = (100.0 - (CELL_SPACING_PER * (VISIBLE_CELL_COUNT + 1) as f32)) / VISIBLE_CELL_COUNT as f32;
const BORDER_WIDTH_PER: f32 = 3.0;
const MAIN_CELL_BORDER_WIDTH_PER: f32 = 5.0;

const TEXT_FONT_SIZE: f32 = 80.0;

mod sandbox;

#[derive(Component)]
struct GameUI;

#[derive(Resource, Deref, DerefMut)]
struct Tape{
    cells: Box<[char; 1_000]>
}

impl Default for Tape{
    fn default() -> Self {
        Self{
            cells: Box::new(['∧'; 1_000])
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_state(GameState::None)
        .insert_resource(Tape::default())
        .add_systems(
        OnEnter(AppState::InGame),
        load_game
        );
    }
}

/// loads the game elements
fn load_game(
    mut commands: Commands,
    tape: ResMut<Tape>,
    game_state: Res<State<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
){
    //loads the tape
    for i in 0..VISIBLE_CELL_COUNT{
        commands.spawn((
            GameUI,
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
            BackgroundColor(Color::NONE),
            Outline::new(Val::Percent(BORDER_WIDTH_PER), Val::Px(0.0), Color::BLACK),
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
        Mesh2d(meshes.add(Triangle2dMeshBuilder::new(Vec2::new(0.0, 100.0),
            Vec2::new(-50.0, 0.0),
            Vec2::new(50.0, 0.0),)
        )),
        MeshMaterial2d(mats.add(Color::BLACK)),
        Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
    ));
    
    match **game_state{
        GameState::Sandbox => sandbox::load(commands, tape),
        _ => println!("unimplemented menu"),
    }
}

///unloads all game elements
fn unload_game(){}