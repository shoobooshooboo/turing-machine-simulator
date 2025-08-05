use bevy::{prelude::*};

use crate::{UpdateSet};

#[derive(Component)]
struct PauseUI;

#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PauseState{
    Paused,
    Unpaused,
}

pub struct PausePlugin;

impl Plugin for PausePlugin{
    fn build(&self, app: &mut App) {
        app
        .insert_state(PauseState::Unpaused)
        .add_systems(OnEnter(PauseState::Paused), load_ui)
        .add_systems(Update, controls.in_set(UpdateSet::Input).run_if(in_state(PauseState::Paused)))
        .add_systems(OnExit(PauseState::Paused), unload_ui)
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
    inputs: Res<ButtonInput<KeyCode>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
){

    //exit paused mode
    if inputs.just_pressed(KeyCode::Escape){
        next_pause_state.set(PauseState::Unpaused);
    }
}

fn unload_ui(
    mut commands: Commands,
    ui_objects: Query<Entity, With<PauseUI>>,
){
    for entity in ui_objects{
        commands.get_entity(entity).unwrap().despawn();
    }
}