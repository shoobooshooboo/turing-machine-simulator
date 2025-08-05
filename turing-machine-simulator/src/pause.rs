use bevy::prelude::*;

use crate::{AppState, UpdateSet};

#[derive(Component)]
struct PauseUI;

pub struct PausePlugin;

impl Plugin for PausePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Paused), load_ui)
        .add_systems(Update, controls.in_set(UpdateSet::Input))
        .add_systems(OnExit(AppState::Paused), unload_ui)
        ;
    }
}

fn load_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
){
    //spawn rectangle that dims the screen
    commands.spawn((
        PauseUI,
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(mats.add(Color::linear_rgba(0.0, 0.0, 0.0, 0.8))),
    ));
}

fn controls(

){

}

fn unload_ui(
    mut commands: Commands,
    ui_objects: Query<Entity, With<PauseUI>>,
){

}