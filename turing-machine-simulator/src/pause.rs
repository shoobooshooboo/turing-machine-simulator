use bevy::{prelude::*};
use crate::{menus::ButtonIndex, BaseFontSize, DefaultFont, UpdateSet};

const PAUSE_MENU_WIDTH_PER: f32 = 60.0;
const PAUSE_MENU_HEIGHT_PER: f32 = 60.0;
const PAUSE_MENU_FONT_SIZE: f32 = 80.0;
const PAUSE_BUTTON_TEXT_FONT_SIZE: f32 = 60.0;
const PAUSE_BUTTON_TEXT_SELECTED_COLOR: Color = Color::WHITE;
const PAUSE_BUTTON_TEXT_UNSELECTED_COLOR: Color = Color::linear_rgb(0.3, 0.3, 0.3);

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
    font: Res<DefaultFont>,
){
    //spawn rectangle that dims the screen
    commands.spawn((
        PauseUI,
        Mesh2d(meshes.add(Rectangle::new(10000.0, 10000.0))),
        MeshMaterial2d(mats.add(Color::linear_rgba(0.0, 0.0, 0.0, 0.8))),
    ));

    commands.spawn((
        PauseUI,
        Node{
            position_type: PositionType::Absolute,
            top: Val::Percent((100.0 - PAUSE_MENU_HEIGHT_PER) / 2.0),
            left: Val::Percent((100.0 - PAUSE_MENU_WIDTH_PER) / 2.0),
            height: Val::Vh(PAUSE_MENU_HEIGHT_PER),
            width: Val::Vw(PAUSE_MENU_WIDTH_PER),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::linear_rgb(0x0B as f32 / 255.0, 0x4F as f32 / 255.0, 0x6C as f32 / 255.0)),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),

    )).with_child((
        Text::new("Paused"),
        TextFont{
            font: font.0.clone(),
            font_size: PAUSE_MENU_FONT_SIZE,
            font_smoothing: bevy::text::FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(PAUSE_MENU_FONT_SIZE),
        TextColor(Color::linear_rgb(1.0, 0.0, 0.0)),
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap()
    )).with_child((
        Button,
        ButtonIndex(0),
        Text::new("Continue"),
        TextFont{
            font: font.0.clone(),
            font_size: PAUSE_BUTTON_TEXT_FONT_SIZE,
            font_smoothing: bevy::text::FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(PAUSE_BUTTON_TEXT_FONT_SIZE),
        TextColor(PAUSE_BUTTON_TEXT_SELECTED_COLOR),
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap()
    )).with_child((
        Button,
        ButtonIndex(0),
        Text::new("Save & Exit"),
        TextFont{
            font: font.0.clone(),
            font_size: PAUSE_BUTTON_TEXT_FONT_SIZE,
            font_smoothing: bevy::text::FontSmoothing::AntiAliased,
            ..Default::default()
        },
        BaseFontSize(PAUSE_BUTTON_TEXT_FONT_SIZE),
        TextColor(PAUSE_BUTTON_TEXT_UNSELECTED_COLOR),
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap()
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