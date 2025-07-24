use bevy::{prelude::*};
use bevy::text::FontSmoothing;

use crate::games::GameState;
use crate::menus::{MenuState, TransitionType};
use crate::{BaseFontSize, menus::{ButtonCount, ButtonIndex, PlayerIndex, BUTTON_OUTLINE_UNSELECTED_WIDTH_PER, BUTTON_UNSELECTED_COLOR, MenuUI}};
//title
const TITLE_HEIGHT_PER: f32 = 30.0;
const TITLE_WIDTH_PER: f32 = 90.0;
const TITLE_FONT_SIZE: f32 = 80.0;
//buttons
const BUTTON_WIDTH_PER: f32 = 60.0;
const BUTTON_HEIGHT_PER: f32 = 12.0;
const BUTTON_OUTLINE_COLOR: Color = Color::BLACK;
const BUTTON_SPACING_PER: f32 = 5.0;
//button text
const BUTTON_TEXT: [&'static str; 2] = ["Sandbox", "Back"];
const BUTTON_TEXT_COLOR: Color = Color::BLACK;
const BUTTON_TEXT_FONT_SIZE: f32 = 60.0;


pub fn load(
    mut commands: Commands,
    mut button_count: ResMut<ButtonCount>,
){
    **button_count = BUTTON_TEXT.len();
    //title text
    commands.spawn((
        MenuUI,
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(TITLE_HEIGHT_PER),
            top: Val::Percent(0.0),
            left: Val::Percent(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        // Transform{
        //     rotation: Quat::from_rotation_z(std::f32::consts::PI / -30.0),
        //     ..Default::default()
        // },
    )).with_child((
        Text::new("Select Gamemode"),
        TextFont{
            font_size: TITLE_FONT_SIZE,
            font_smoothing: FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(TITLE_FONT_SIZE),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
    ));
    //make buttons
    for i in 0..BUTTON_TEXT.len(){
        commands.spawn((
            MenuUI,
            Button,
            ButtonIndex(i),
            Node{
                position_type: PositionType::Absolute,
                width: Val::Percent(BUTTON_WIDTH_PER),
                height: Val::Percent(BUTTON_HEIGHT_PER),
                bottom: Val::Percent(100.0 - TITLE_HEIGHT_PER - (BUTTON_HEIGHT_PER + BUTTON_SPACING_PER) * (i + 1) as f32),
                left: Val::Percent((100.0 - BUTTON_WIDTH_PER) / 2.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(BUTTON_UNSELECTED_COLOR),
            BorderRadius::all(Val::VMax(5.0)),
            Outline{
                color: BUTTON_OUTLINE_COLOR,
                width: Val::Percent(BUTTON_OUTLINE_UNSELECTED_WIDTH_PER),
                ..Default::default()
            },
        )).with_child((
            Text::new(BUTTON_TEXT[i]),
            TextFont {
                font_size: BUTTON_TEXT_FONT_SIZE,
                font_smoothing: FontSmoothing::AntiAliased,
                ..Default::default()
            },
            BaseFontSize(BUTTON_TEXT_FONT_SIZE),
            TextColor(BUTTON_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
    }
}

pub fn transition(
    player_index: ResMut<PlayerIndex>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut _next_game_state: ResMut<NextState<GameState>>,
) -> TransitionType{
    match **player_index{
        0 => {next_menu_state.set(MenuState::SandboxMenu);},
        1 => next_menu_state.set(MenuState::MainMenu),
        _ => panic!("somehow went into a non-existant menu"),
    }

    if **player_index == 1{
        TransitionType::Out
    }
    else{
        TransitionType::In
    }
}

pub fn detransition(
    mut next_menu_state: ResMut<NextState<MenuState>>, 
){
    next_menu_state.set(MenuState::MainMenu);
}