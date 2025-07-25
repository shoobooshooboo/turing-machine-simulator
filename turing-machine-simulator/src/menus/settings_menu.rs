use bevy::{prelude::*};
use bevy::text::FontSmoothing;

use crate::{BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH};
use crate::{menus::{PlayerIndex, TransitionType}, BaseFontSize, MenuState};

use super::{MenuUI, ButtonIndex, ButtonCount, BUTTON_OUTLINE_UNSELECTED_WIDTH_PER, BUTTON_UNSELECTED_COLOR};

//text
const TEXT_HEIGHT_PER: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 100.0;
const SPACING: f32 = 5.0;  
//subtext
const SUBTEXT_HEIGHT_PER: f32 = TEXT_HEIGHT_PER / 2.0;
const SUBTEXT_FONT_SIZE: f32 = TEXT_FONT_SIZE / 2.0;
//button
const BUTTON_WIDTH_PER: f32 = 60.0;
const BUTTON_HEIGHT_PER: f32 = 12.0;
const BUTTON_OUTLINE_COLOR: Color = Color::BLACK;
//button text
const BUTTON_TEXT_COLOR: Color = Color::BLACK;
const BUTTON_TEXT_FONT_SIZE: f32 = 60.0;
//sliders
const SLIDER_HEIGHT: f32 = BASE_WINDOW_HEIGHT * 0.03;
const SLIDER_WIDTH: f32 = BASE_WINDOW_WIDTH * 0.9 / 2.0;
const SLIDER_COLOR: Color = Color::BLACK;
//slider thumb
const SLIDER_THUMB_HEIGHT: f32 = SLIDER_HEIGHT * 3.0;
const SLIDER_THUMB_WIDTH: f32 = SLIDER_WIDTH / 100.0;
const SLIDER_THUMB_COLOR: Color = Color::WHITE;
//slider text
const SLIDER_TEXT_COLOR: Color = Color::WHITE;
const SLIDER_TEXT_FONT_SIZE: f32 = 30.0;
const SLIDER_TEXT: [&'static str; 1] = ["Master Volume"];

#[derive(Component, Deref)]
pub struct Slider(usize);

#[derive(Component, Deref)]
pub struct Thumb(usize);

pub fn load(
    mut commands: Commands,
    mut button_count: ResMut<ButtonCount>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<ColorMaterial>>,
){
    **button_count = SLIDER_TEXT.len() + 1;
    //TEXT
    commands.spawn((
        MenuUI,
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
    )).with_child((
        Text::new("Settings"),
        TextFont{
            font_size: TEXT_FONT_SIZE,
            font_smoothing: FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(TEXT_FONT_SIZE),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
    ));

    //Sliders
    for i in 0..SLIDER_TEXT.len(){
        //slider bar
        commands.spawn((
            MenuUI,
            Slider(i),
            Mesh2d(meshes.add(Rectangle::new(SLIDER_WIDTH, SLIDER_HEIGHT))),
            MeshMaterial2d(mats.add(SLIDER_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 110.0, 0.0))
        ))
        //slider thumb
        .with_child((
            Thumb(i),
            Mesh2d(meshes.add(Rectangle::new(SLIDER_THUMB_WIDTH, SLIDER_THUMB_HEIGHT))),
            MeshMaterial2d(mats.add(SLIDER_COLOR)),
            Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
        ));
        commands.spawn((
            MenuUI,
            Node{
                position_type: PositionType::Absolute,
                width: Val::Percent(BASE_WINDOW_WIDTH / SLIDER_WIDTH),
                height: Val::Percent(BASE_WINDOW_HEIGHT / SLIDER_THUMB_HEIGHT),
                top: Val::Percent((BASE_WINDOW_HEIGHT / SLIDER_THUMB_HEIGHT + SPACING) * i as f32 + TEXT_HEIGHT_PER),
                left: Val::Percent(10.0 + BASE_WINDOW_WIDTH / SLIDER_WIDTH),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
        )).with_child((
            Text::new(SLIDER_TEXT[i]),
            TextFont{
                font_size: SLIDER_TEXT_FONT_SIZE,
                font_smoothing: FontSmoothing::AntiAliased,
                ..Default::default()
            },
            BaseFontSize(SLIDER_TEXT_FONT_SIZE),
            TextColor(SLIDER_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    }
 
    //EXIT BUTTON
    commands.spawn((
            MenuUI,
            Button,
            ButtonIndex(SLIDER_TEXT.len()),
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
    player_index: ResMut<PlayerIndex>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) -> TransitionType{
    if **player_index == SLIDER_TEXT.len(){ 
        next_menu_state.set(MenuState::MainMenu);
        TransitionType::Out
    }else{
        TransitionType::In
    }
}

pub fn detransition(
    mut next_menu_state: ResMut<NextState<MenuState>>, 
){
    next_menu_state.set(MenuState::MainMenu);
}

pub fn update_sliders(
    player_index: Res<PlayerIndex>,
    thumbs: Query<(&Thumb, &MeshMaterial2d<ColorMaterial>)>,
    mut mats: ResMut<Assets<ColorMaterial>>,
){
    for (index,  bgc) in & thumbs{
        if **player_index == **index{
            mats.get_mut(bgc.id()).unwrap().color = Color::WHITE;
        }else{
            mats.get_mut(bgc.id()).unwrap().color = SLIDER_COLOR;
        }
    } 
}