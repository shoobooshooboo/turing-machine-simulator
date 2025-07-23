use std::fs;
use std::path::Path;
use bevy::prelude::*;

use crate::games::{SaveFileIndex, Tape, SAVE_FILE_PATH};

pub fn load(
    mut _commands: Commands,
    save_file_index: Res<SaveFileIndex>,
    mut tape: ResMut<Tape>,
){
    let contents = match fs::read_to_string(Path::new(&format!("{}{}.sav", SAVE_FILE_PATH, save_file_index.clone().unwrap()))){
        Ok(s) => s,
        Err(_) => "ERROR LOADING FILE".to_string(),
    };

    for (i, c) in contents.char_indices(){
        tape[i] = c;
    }
}