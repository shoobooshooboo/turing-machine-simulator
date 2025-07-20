use bevy::prelude::*;
const TITLE_HEIGHT_PER: f32 = 30.0;
const TITLE_WIDTH_PER: f32 = 90.0;
const BUTTON_WIDTH_PER: f32 = 60.0;
const BUTTON_HEIGHT_PER: f32 = 12.0;
const BUTTON_UNSELECTED_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.5);
const BUTTON_SELECTED_COLOR: Color = Color::linear_rgb(0.75, 0.75, 0.75);
const BUTTON_TEXT_COLOR: Color = Color::BLACK;
const BUTTON_SPACING_PER: f32 = 5.0;
const BUTTON_TEXT: [&'static str; 4] = ["Play Game!", "Settings", "Credits", "Quit"];

pub fn startup(
    mut commands: Commands,
){
    commands.spawn((
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
        }

    )).with_child((
        Text::new("Turing Machine Simulator!"),
        TextFont{
            font_size: 80.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextShadow{
            color: Color::linear_rgba(0.9, 0.9, 0.9, 0.8),
            offset: Vec2 { x: -2.0, y: 1.0 }
        },
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
    ));
    for i in 0..BUTTON_TEXT.len(){
        commands.spawn((
            Button,
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
        )).with_child((
            Text::new(BUTTON_TEXT[i]),
            TextFont {
                font_size: 60.0,
                ..Default::default()
            },
            TextColor(BUTTON_TEXT_COLOR),
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
    }
}