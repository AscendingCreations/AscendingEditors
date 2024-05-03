use graphics::*;

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;

use winit::{event::*, keyboard::*};

use crate::interface::preference::keybind::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub key_code: Vec<Key>,
    pub key_code_modifier: Vec<[bool; 3]>,
    pub hide_fps: bool,
    pub hide_tileset_bg: bool,
    pub hide_mapview_bg: bool,
    pub map_selection_color: [u8; 4],
    pub tile_selection_color: [u8; 4],
    pub save_json: bool,
}

impl ConfigData {
    pub fn default() -> Self {
        let mut key_code = Vec::new();
        let mut key_code_modifier = Vec::new();

        for key in 0..EditorKey::Count as usize {
            let keycode = match key {
                1 => Key::Character(SmolStr::new("s")), // Save
                2 => Key::Character(SmolStr::new("z")), // Undo
                3 => Key::Character(SmolStr::new("y")), // Redo
                4 => Key::Character(SmolStr::new("d")), // Draw
                5 => Key::Character(SmolStr::new("e")), // Erase
                6 => Key::Character(SmolStr::new("f")), // Fill
                7 => Key::Character(SmolStr::new("y")), // Eyetool
                _ => Key::Character(SmolStr::new("o")), // Load
            };
            let keycodemodifier = match key {
                1 => [true, false, false],  // Save
                2 => [true, false, false],  // Undo
                3 => [true, false, false],  // Redo
                4 => [false, false, false], // Draw
                5 => [false, false, false], // Erase
                6 => [false, false, false], // Fill
                7 => [false, false, false], // Eyetool
                _ => [true, false, false],  // Load
            };
            key_code.push(keycode);
            key_code_modifier.push(keycodemodifier);
        }

        Self {
            key_code,
            key_code_modifier,
            hide_fps: false,
            hide_tileset_bg: false,
            hide_mapview_bg: false,
            map_selection_color: [0, 0, 150, 150],
            tile_selection_color: [80, 0, 0, 150],
            save_json: false,
        }
    }

    pub fn save_config(&self) -> Result<(), GraphicsError> {
        let name = "./map_config.json".to_string();

        match OpenOptions::new().truncate(true).write(true).open(&name) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(&file, self) {
                    Err(GraphicsError::Other(OtherError::new(&format!(
                        "Serdes File Error Err {:?}",
                        e
                    ))))
                } else {
                    Ok(())
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(GraphicsError::Other(OtherError::new(&format!(
                "Failed to open {}, Err {:?}",
                name, e
            )))),
        }
    }

    pub fn reset_config(&mut self) {
        let default_config = ConfigData::default();
        *self = default_config;
    }

    pub fn set_data(&mut self, data: ConfigData) {
        *self = data;
    }
}

pub fn create_config(data: &ConfigData) -> Result<(), GraphicsError> {
    let name = "./map_config.json".to_string();

    match OpenOptions::new().write(true).create_new(true).open(&name) {
        Ok(file) => {
            if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                Err(GraphicsError::Other(OtherError::new(&format!(
                    "Serdes File Error Err {:?}",
                    e
                ))))
            } else {
                Ok(())
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(GraphicsError::Other(OtherError::new(&format!(
            "Failed to open {}, Err {:?}",
            name, e
        )))),
    }
}

pub fn load_config() -> ConfigData {
    if !is_config_exist() {
        let data = ConfigData::default();
        match create_config(&ConfigData::default()) {
            Ok(()) => return data,
            Err(_) => return ConfigData::default(),
        }
    }

    match OpenOptions::new().read(true).open("./map_config.json") {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error {:?}", e);
                    ConfigData::default()
                }
            }
        }
        Err(_) => ConfigData::default(),
    }
}

pub fn is_config_exist() -> bool {
    let name = "./map_config.json".to_string();
    Path::new(&name).exists()
}
