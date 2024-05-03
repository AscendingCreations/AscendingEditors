use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigData {
    pub save_json: bool,
}

pub fn create_config(data: &ConfigData) -> Result<(), String> {
    let name = "./shop_config.json".to_string();

    match OpenOptions::new().write(true).create_new(true).open(&name) {
        Ok(file) => {
            if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                Err(format!("Serdes File Error Err {:?}", e))
            } else {
                Ok(())
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
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

    match OpenOptions::new().read(true).open("./shop_config.json") {
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
    let name = "./shop_config.json".to_string();
    Path::new(&name).exists()
}
