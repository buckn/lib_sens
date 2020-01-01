use crate::games_enum::SupportedGames;
use crate::platform::Platform;
use crate::profile::SensProfile;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::env;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    profiles: Vec<SensProfile>,
    steam_paths: SteamFolders,
    platform: Platform,
}

impl Profiles {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            steam_paths: SteamFolders::new(),
            platform: Platform::new(),
        }
    }

    pub fn set_game_sens_in_profile(
        &mut self,
        game: SupportedGames,
        sens: f64,
        profile_index: i32,
        ) {
        self.profiles[profile_index as usize].set_game_sens(game, sens);
    }
    pub fn equalize_profile_at_index(&mut self, game: SupportedGames) {
        for i in 0..self.profiles.len() {
            self.profiles[i].equalize(game);
        }
    }
    pub fn set_platform(&mut self) {
        self.platform = Platform::new();
    }
    pub fn len(&self) -> usize {
        self.profiles.len()
    }
    pub fn add_profile(&mut self) {
        self.profiles.push(SensProfile::new());
    }
    pub fn remove_profile(&mut self, index: i32) {
        self.profiles.remove(index as usize);
    }
    pub fn switch_profile(&self, index: i32) {
        self.profiles[index as usize].clone().save_all_to_configs();
    }
    pub fn get_steam_folder_at_pointer(&self, x: usize) -> String {
        self.steam_paths.get_steam_folder_at_pointer(x)
    }
    pub fn add_steam_folder(&mut self, x: String) {
        self.steam_paths.add_steam_folder(x);
    }
    pub fn remove_steam_folder_at_index(&mut self, x: usize) {
        self.steam_paths.remove_steam_folder_at_index(x)
    }
    pub fn set_paths(&mut self) {
        for profile in self.profiles.iter_mut() {
            profile.set_all_paths(self.steam_paths.clone(), self.platform.clone());
        }
    }
    pub fn fs_read_game_sens_at_index(&mut self, game: SupportedGames, index: i32) {
        self.profiles[index as usize].fs_read_game_sens(game);
    }
    pub fn fs_read_all_game_sens_at_index(mut self, index: i32) {
        self.profiles[index as usize].fs_read_all_game_sens();
    }

    pub fn save_json(&self) {
        let homepath: String = env::home_dir().unwrap().display().to_string();
        
        if !(Path::new(&(homepath.clone() + "/sens/profiles.json")).exists()) {
            fs::create_dir(homepath.clone() + "/sens").unwrap();
            File::create(homepath.clone() + "/sens/profiles.json").unwrap().write_all(b"").unwrap();
        }

        let mut file_write = File::create(homepath + "/sens/profiles.json").unwrap();
        file_write.write(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
    }
    pub fn load_json() -> Self {
        let homepath: String = env::home_dir().unwrap().display().to_string();
        
        if !(Path::new(&(homepath.clone() + "/sens/profiles.json")).exists()) {
            fs::create_dir(homepath.clone() + "/sens").unwrap();
            File::create(homepath.clone() + "/sens/profiles.json").unwrap().write_all(b"").unwrap();
        }

        let file = File::open(homepath + "/sens/profiles.json");
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}