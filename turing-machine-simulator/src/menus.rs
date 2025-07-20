use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::{GameState, BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH};

use super::MenuState;
const BUTTON_UNSELECTED_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const BUTTON_SELECTED_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BUTTON_OUTLINE_UNSELECTED_WIDTH_PER: f32 = 0.5;
const BUTTON_OUTLINE_SELECTED_WIDTH_PER: f32 = 0.75;

#[derive(Component)]
pub struct UI;

#[derive(Component, Deref, DerefMut)]
pub struct ButtonIndex(usize);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PlayerIndex(usize);

#[derive(Component, Deref, DerefMut)]
pub struct BaseFontSize(f32);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ButtonCount(usize);

pub mod main_menu;
pub mod credits_menu;

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

pub fn button_selection(
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

pub fn controls(
    mut player_index: ResMut<PlayerIndex>,
    inputs: Res<ButtonInput<KeyCode>>,
    exit: EventWriter<AppExit>,
    menu_state: Res<State<MenuState>>,
    next_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,    
    button_count: Res<ButtonCount>,

){
    if inputs.just_pressed(KeyCode::ArrowUp){
        **player_index = player_index.checked_sub(1).unwrap_or(**button_count - 1);
    }else if inputs.just_pressed(KeyCode::ArrowDown){
        **player_index = (**player_index + 1) % **button_count;
    }
    **player_index = player_index.clamp(0, **button_count - 1);

    if inputs.just_pressed(KeyCode::Enter){
        next_game_state.set(GameState::Transition);
        match **menu_state{
            MenuState::MainMenu => main_menu::transition(player_index, exit, next_state),
            MenuState::CreditsMenu => credits_menu::transition(next_state),
            _ => println!("unimplemented menu"),
        }
    }
}

pub fn unload_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<UI>>,
){
    for entity in ui_elements{
        commands.entity(entity).despawn();
    }
}

pub fn load_ui(
    commands: Commands,
    button_count: ResMut<ButtonCount>,
    menu_state: Res<State<MenuState>>,
){
    match **menu_state{
        MenuState::MainMenu => main_menu::load(commands, button_count),
        MenuState::CreditsMenu => credits_menu::load(commands, button_count),
        _ => print!("unimplemented menu"),
    }
}