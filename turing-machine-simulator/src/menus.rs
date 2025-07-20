use bevy::prelude::*;
use super::GameState;

#[derive(Component)]
pub struct UI;

#[derive(Component, Deref, DerefMut)]
pub struct ButtonIndex(usize);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerIndex(usize);

pub mod main_menu;