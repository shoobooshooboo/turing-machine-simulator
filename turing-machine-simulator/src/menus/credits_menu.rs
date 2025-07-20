use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::MenuState;

use super::{UI, ButtonIndex, ButtonCount, BaseFontSize, BUTTON_OUTLINE_UNSELECTED_WIDTH_PER, BUTTON_UNSELECTED_COLOR};

//text
const TEXT_HEIGHT_PER: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 100.0;
const TEXT_SPACING_PER: f32 = 5.0;  
//subtext
const SUBTEXT_HEIGHT_PER: f32 = TEXT_HEIGHT_PER / 2.0;
const SUBTEXT_FONT_SIZE: f32 = TEXT_FONT_SIZE / 2.0;
//button
const BUTTON_WIDTH_PER: f32 = 60.0;
const BUTTON_HEIGHT_PER: f32 = 12.0;
const BUTTON_OUTLINE_COLOR: Color = Color::BLACK;
const BUTTON_SPACING_PER: f32 = 5.0;
//button text
const BUTTON_TEXT_COLOR: Color = Color::BLACK;
const BUTTON_TEXT_FONT_SIZE: f32 = 60.0;

pub fn load(
    mut commands: Commands,
    mut button_count: ResMut<ButtonCount>,
){
    **button_count = 1;
    //TEXT
    commands.spawn((
        UI,
        Node{
            width: Val::Percent(95.0),
            height: Val::Percent(TEXT_HEIGHT_PER),
            top: Val::Percent(0.0),
            left: Val::Percent(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        Transform{
            // rotation: Quat::from_rotation_z(std::f32::consts::PI / -30.0),
            ..Default::default()
        },
    )).with_child((
        Text::new("Everything: Noel vanSchaick"),
        TextFont{
            font_size: TEXT_FONT_SIZE,
            font_smoothing: FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(TEXT_FONT_SIZE),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Left),
    ));

    //SUBTEXT
    commands.spawn((
        UI,
        Node{
            width: Val::Percent(95.0),
            height: Val::Percent(SUBTEXT_HEIGHT_PER),
            top: Val::Percent(TEXT_HEIGHT_PER + TEXT_SPACING_PER),
            left: Val::Percent(0.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            ..Default::default()
        },
        BackgroundColor(Color::NONE),
        Transform{
            // rotation: Quat::from_rotation_z(std::f32::consts::PI / -30.0),
            ..Default::default()
        },
    )).with_child((
        Text::new("Enabler: H Hays"),
        TextFont{
            font_size: SUBTEXT_FONT_SIZE,
            font_smoothing: FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(SUBTEXT_FONT_SIZE),
        TextColor(Color::linear_rgba(0.75, 0.75, 0.75, 0.75)),
        TextLayout::new_with_justify(JustifyText::Left),
    ));

    //EXIT BUTTON
    commands.spawn((
            UI,
            Button,
            ButtonIndex(0),
            Node{
                position_type: PositionType::Absolute,
                width: Val::Percent(BUTTON_WIDTH_PER),
                height: Val::Percent(BUTTON_HEIGHT_PER),
                bottom: Val::Percent(2.0),
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
            Text::new("Back"),
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

pub fn transition(
    mut next_state: ResMut<NextState<MenuState>>, 
){
    next_state.set(MenuState::MainMenu)
}