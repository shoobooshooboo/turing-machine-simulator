use bevy::prelude::*;
use crate::{AppState, GameState, MenuState};

const BUTTON_UNSELECTED_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const BUTTON_SELECTED_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const BUTTON_OUTLINE_UNSELECTED_WIDTH_PER: f32 = 0.5;
const BUTTON_OUTLINE_SELECTED_WIDTH_PER: f32 = 0.75;

#[derive(Component)]
struct MenuUI;

#[derive(Component, Deref, DerefMut)]
struct ButtonIndex(usize);

#[derive(Resource, Deref, DerefMut, Default)]
struct PlayerIndex(usize);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ButtonCount(usize);

mod main_menu;
mod credits_menu;
mod game_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App){
    app
    .insert_state(MenuState::MainMenu)
    .insert_resource(PlayerIndex::default())
    .insert_resource(ButtonCount::default())
    .add_systems(
        OnEnter(AppState::InMenu),
        load_ui
    )
    .add_systems(
        OnTransition{exited: AppState::InMenu, entered: AppState::Transition},
         unload_ui
    )
    .add_systems(
    Update,
    (
        controls.run_if(in_state(AppState::InMenu)),
        button_selection.run_if(in_state(AppState::InMenu)),
    ).chain());
    }
}

/// sets a button's background color and border width depending off it is selected or not
fn button_selection(
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

/// handles controls while in the menu
fn controls(
    mut player_index: ResMut<PlayerIndex>,
    inputs: Res<ButtonInput<KeyCode>>,
    exit: EventWriter<AppExit>,
    menu_state: Res<State<MenuState>>,
    next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,    
    next_game_state: ResMut<NextState<GameState>>,    
    button_count: Res<ButtonCount>,

){
    if inputs.just_pressed(KeyCode::ArrowUp){
        **player_index = player_index.checked_sub(1).unwrap_or(**button_count - 1);
    }else if inputs.just_pressed(KeyCode::ArrowDown){
        **player_index = (**player_index + 1) % **button_count;
    }
    **player_index = player_index.clamp(0, **button_count - 1);

    if inputs.just_pressed(KeyCode::Enter){
        next_app_state.set(AppState::Transition);
        match **menu_state{
            MenuState::MainMenu => main_menu::transition(player_index, exit, next_menu_state),
            MenuState::GameMenu => game_menu::transition(player_index, next_menu_state, next_game_state),
            MenuState::CreditsMenu => credits_menu::transition(next_menu_state),
            _ => println!("unimplemented menu"),
        }
    }
}

/// unloads all menu ui elements
fn unload_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<MenuUI>>,
){
    for entity in ui_elements{
        commands.entity(entity).despawn();
    }
}

/// loads all menu ui elements
fn load_ui(
    commands: Commands,
    button_count: ResMut<ButtonCount>,
    menu_state: Res<State<MenuState>>,
){
    match **menu_state{
        MenuState::MainMenu => main_menu::load(commands, button_count),
        MenuState::GameMenu => game_menu::load(commands, button_count), 
        MenuState::CreditsMenu => credits_menu::load(commands, button_count),
        _ => print!("unimplemented menu"),
    }
}