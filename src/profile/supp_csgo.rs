use crate::game_trait::Game;
use crate::platform::*;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CSGO {
    sens: f64,
    path: String,
}

impl Game for CSGO {
    fn new() -> Self {
        Self {
            sens: 1.0,
            path: "".to_string(),
        }
    }
    fn get_sens(&self) -> f64 {
        self.sens
    }
    fn set_sens(&mut self, value: f64) {
        if value > 0.0 {
            self.sens = value;
        }
    }
    fn convert_to_csgo(value: f64) -> f64 {
        value
    }
    fn convert_from_csgo(value: f64) -> f64 {
        value
    }
    fn fs_read(&self) -> f64 {
        let mut return_val: f64 = 0.0;
        let file = File::open(&self.path);
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents).unwrap();

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
        return_val
    }
    fn fs_write(self) {
        let mut y = "".to_string();

        let file = File::open(self.path.clone());
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents).unwrap();

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

        let mut file_write = File::create(self.path).unwrap();
        file_write.write(y.as_bytes()).unwrap();
    }
    fn get_path(&self) -> String {
        let return_path = String::from(&self.path);
        return_path
    }
    fn set_path(&mut self, steam_paths: SteamFolders, _platform_value: Platform) {
        self.path = steam_paths
            .find_file_in_steam_paths("userdata/310883639/730/local/cfg/config.cfg".to_string())
            .unwrap();
    }
}
