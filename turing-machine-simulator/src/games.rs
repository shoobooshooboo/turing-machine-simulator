use bevy::prelude::*;

use crate::GameState;

pub mod sandbox;

#[derive(Component)]
pub struct GameUI;

#[derive(Resource, Deref, DerefMut)]
pub struct Tape{
    cells: [char; 1_000_000]
}

pub fn load(
    mut commands: Commands,
    tape: ResMut<Tape>,
    game_state: Res<State<GameState>>,
){
    
}

pub fn unload(){}