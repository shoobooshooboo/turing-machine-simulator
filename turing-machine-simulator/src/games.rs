use bevy::{image::TranscodeFormat, prelude::*, render::mesh::Triangle2dMeshBuilder, sprite::Material2d, window::PrimaryWindow};

use crate::{AppState, GameState};

pub mod sandbox;

#[derive(Component)]
pub struct GameUI;

#[derive(Resource, Deref, DerefMut)]
pub struct Tape{
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
        load
        );
    }
}

/// loads the game elements
pub fn load(
    mut commands: Commands,
    tape: ResMut<Tape>,
    game_state: Res<State<GameState>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
){
    //loads the tape


    //loads cursor
    println!("spawning cursor");
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
pub fn unload(){}