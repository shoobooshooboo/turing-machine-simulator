use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::{menus::{ButtonCount, ButtonIndex, MenuUI, PlayerIndex, TransitionType, BUTTON_OUTLINE_UNSELECTED_WIDTH_PER, BUTTON_UNSELECTED_COLOR}, BaseFontSize, MenuState};
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
const BUTTON_TEXT: [&'static str; 4] = ["Play Game!", "Settings", "Credits", "Quit"];
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
        Transform{
            rotation: Quat::from_rotation_z(std::f32::consts::PI / -30.0),
            ..Default::default()
        },
    )).with_child((
        Text::new("Turing Machine Simulator!"),
        TextFont{
            font_size: TITLE_FONT_SIZE,
            font_smoothing: FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(TITLE_FONT_SIZE),
        TextColor(Color::WHITE),
        TextShadow{
            color: Color::linear_rgba(0.9, 0.9, 0.9, 0.8),
            offset: Vec2 { x: -2.0, y: 1.0 }
        },
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
    mut exit: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) -> TransitionType{
    match **player_index{
        0 => {next_menu_state.set(MenuState::GameMenu)},
        1 => {next_menu_state.set(MenuState::SettingsMenu)},
        2 => {next_menu_state.set(MenuState::CreditsMenu)},
        3 => {exit.write(AppExit::Success);},
        _ => panic!("somehow went into a non-existant menu"),
    }

    if **player_index == 3{
        TransitionType::Out
    }
    else{
        TransitionType::In
    }
}