use crate::game_trait::Game;
use crate::platform::*;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct CSGO {
    sens: f64,
    path: String,
}

impl CSGO {
    pub fn new() -> Self {
        Self {
            sens: 1.0,
            path: "".to_string(),
        }
    }
}

#[typetag::serde]
impl Game for CSGO {
    fn to_string(&self) -> String {
        "CSGO".to_owned() + ":    " + &self.sens.to_string() + "\n"
    }
    fn get_sens(&self) -> f64 {
        self.sens
    }
    fn set_sens(&mut self, value: f64) {
        if value > 0.0 {
            self.sens = value;
        }
    }
    fn set_sens_from_csgo(&mut self, value: f64) {
        if value > 0.0 {
            self.sens = self.convert_self_to_csgo();
        }
    }
    fn set_sens_to_fs_value(&mut self) -> Result<(), io::Error> {
        self.set_sens(self.fs_read()?);
        Ok(())
    }
    fn convert_self_to_csgo(&self) -> f64 {
        self.sens
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
        let mut y = "".to_string();

        let file = File::open(self.path.clone());
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents)?;

        let replaced = contents
            .replace("\n", "\n(*)")
            .replace(" ", " (*)")
            .replace("   ", "    (*)");

        let mut words = replaced.split("(*)").collect::<Vec<_>>();

        let change_to = "\"".to_owned() + &self.sens.to_string() + &"\"".to_owned() + "\n";

        let mut iter = words
            .iter_mut()
            .skip_while(|e| e.to_string() != "sensitivity ".to_string());
        let _ = iter.next();
        let value_after = iter.next().expect("File ended after sensitivity!");
        *value_after = &change_to;

        for item in words.iter() {
            y += item;
        }

        let mut file_write = File::create(self.path.clone())?;
        file_write.write(y.as_bytes())?;
        Ok(())
    }
    fn get_path(&self) -> String {
        let return_path = String::from(&self.path);
        return_path
    }
    fn set_path(
        &mut self,
        steam_paths: SteamFolders,
        _platform_value: Platform,
    ) -> Result<(), io::Error> {
        self.path = steam_paths
            .find_file_in_steam_paths_with_id("/730/remote/cfg/config.cfg".to_string())?;
        Ok(())
    }
}
