use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::{BASE_WINDOW_ASPECT_RATIO, BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH};

use super::GameState;

#[derive(Component)]
pub struct UI;

#[derive(Component, Deref, DerefMut)]
pub struct ButtonIndex(usize);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerIndex(usize);

#[derive(Component, Deref, DerefMut)]
pub struct BaseFontSize(f32);

pub mod main_menu;

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