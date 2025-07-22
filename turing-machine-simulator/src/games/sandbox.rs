use bevy::prelude::*;

use crate::games::Tape;

pub fn load(
    mut _commands: Commands,
    mut tape: ResMut<Tape>,
){
    for (i, c) in "hello world".char_indices(){
        tape[i] = c;
    }
}