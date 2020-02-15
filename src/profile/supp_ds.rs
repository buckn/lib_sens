use crate::game_trait::Game;
use crate::platform::*;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read};
use std::io::{prelude::*, BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct DS {
    sens: f64,
    path: String,
}

impl DS {
    const CONVERSION_FACTOR: f64 = 0.1706;

    pub fn new() -> Self {
        Self {
            sens: 1.0,
            path: "".to_string(),
        }
    }
}

#[typetag::serde]
impl Game for DS {
    fn to_string(&self) -> String {
        "Dead Space".to_owned() + ":    " + &self.sens.to_string() + "\n"
    }
    fn get_sens(&self) -> f64 {
        self.sens
    }
    fn set_sens(&mut self, value: f64) {
        if value > 0.0 {
            self.sens = value;
        }
    }
    fn set_sens_to_fs_value(&mut self) -> Result<(), io::Error> {
        self.set_sens(self.fs_read()?);
        Ok(())
    }
    fn set_sens_from_csgo_sens(&mut self, value: f64) {
        if value > 0.0 {
            self.sens = value * DS::CONVERSION_FACTOR;
        }
    }
    fn convert_self_to_csgo(&self) -> f64 {
        self.sens / DS::CONVERSION_FACTOR
    }

    fn fs_read(&self) -> Result<f64, io::Error> {
        let mut return_val: f64 = 0.0;
        let mut file = File::open(&self.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let key_to_find = "sensitivity";

        let mut values = contents
        .lines()
        .filter_map(|l| {
            if let [key, value] = l.split_whitespace().collect::<Vec<_>>()[..] {
                Some((key, value[1..value.len() - 1].parse::<String>().unwrap()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

        for (key, value) in values.iter_mut() {
            if key != &"" && key_to_find.contains(key) {
                return_val = value.parse::<f64>().unwrap();
            }
        }
        Ok(return_val)
    }
    fn fs_write(&self) -> Result<(), io::Error> {
        let file = File::open(self.path.clone())?;
        let reader = BufReader::new(file);
        let mut write_string: String = "".to_string();
        let mut current_line;

        for line in reader.lines() {
            current_line = line.unwrap();
            if current_line.clone().starts_with("Control.MouseSensitivity = ") {
                write_string += &(("Control.MouseSensitivity = ".to_owned() + &self.sens.to_string()) + &"\n".to_string());
            } else {
                write_string += &(current_line + &"\n".to_string());
            }
        }

        let mut file_write = File::create(self.path.clone())?;
        file_write.write(write_string.as_bytes())?;
        Ok(())
    }
    fn get_path(&self) -> String {
        String::from(&self.path)
    }
    fn set_path(
        &mut self,
        steam_paths: &SteamFolders,
        _platform_value: Platform,
        ) -> Result<(), io::Error> {
        self.path = steam_paths
        .find_file_in_steam_paths("steamapps/compatdata/17470/pfx/drive_c/users/steamuser/Local Settings/Application Data/Electronic Arts/Dead Space/settings.txt".to_string())?;
        Ok(())
    }
}
