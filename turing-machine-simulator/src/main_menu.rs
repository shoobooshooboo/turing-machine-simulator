use bevy::prelude::*;
const BUTTON_WIDTH_PER: f32 = 60.0;
const BUTTON_HEIGHT_PER: f32 = 20.0;
const BUTTON_UNSELECTED_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.5);
const BUTTON_TEXT_COLOR: Color = Color::BLACK;

pub fn startup(
    mut commands: Commands,
){
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            width: Val::Percent(BUTTON_WIDTH_PER),
            height: Val::Percent(BUTTON_HEIGHT_PER),
            bottom: Val::Percent(50.0 - (BUTTON_HEIGHT_PER / 2.0)),
            left: Val::Percent((100.0 - BUTTON_WIDTH_PER) / 2.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(BUTTON_UNSELECTED_COLOR),
        BorderRadius::all(Val::VMax(5.0)),
    )).with_child((
        Text::new("Hello world!"),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
        TextColor(BUTTON_TEXT_COLOR),
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
    ));
}