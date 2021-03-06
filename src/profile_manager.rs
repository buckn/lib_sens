use crate::games_enum::SupportedGames;
use crate::platform::Platform;
use crate::profile::SensProfile;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

extern crate dirs;

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    profiles: Vec<SensProfile>,
    steam_paths: SteamFolders,
    platform: Platform,
}

impl Profiles {
    ///This creates a new set of profiles with default values.
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            steam_paths: SteamFolders::new(),
            platform: Platform::new(),
        }
    }
    ///Sets a game sensitivity in a profile at a specific index, for a specific game.
    pub fn set_game_sens_in_profile(
        &mut self,
        game: SupportedGames,
        sens: f64,
        profile_index: i32,
    ) {
        self.profiles[profile_index as usize].set_game_sens(game, sens);
    }
    ///This takes a profile at a specific index and converts the sensitivities so that they are all equivalent to one game.
    pub fn equalize_profile_at_index(&mut self, game: SupportedGames, index: i32) {
        self.profiles[index as usize].equalize(game);
    }
    ///This automatically detects your platform and sets it accordingly.
    pub fn set_platform(&mut self) {
        self.platform = Platform::new();
    }
    ///This gets the length of the profiles as usize.
    pub fn len(&self) -> usize {
        self.profiles.len()
    }
    ///This appends a profile to the profiles vector.
    pub fn add_profile(&mut self) {
        self.profiles.push(SensProfile::new());
    }
    ///This removes a profile at a specific index.
    pub fn remove_profile(&mut self, index: i32) {
        self.profiles.remove(index as usize);
    }
    ///This saves the sensitivity values in a specific profile to local configuration files.
    pub fn switch_profile(&self, index: i32) -> Result<(), io::Error> {
        self.profiles[index as usize].save_all_to_configs()?;
        Ok(())
    }
    ///This gets a specific steam folder from the vector of steam folders.
    pub fn get_steam_folder_at_index(&self, index: i32) -> String {
        self.steam_paths.get_steam_folder_at_pointer(index as usize)
    }
    ///This appends a steam folder to the vector of steam folders.
    pub fn add_steam_folder(&mut self, path_string: String) {
        self.steam_paths.add_steam_folder(path_string);
    }
    ///This removes a specific steam folder from the steam folders vector.
    pub fn remove_steam_folder_at_index(&mut self, index: i32) {
        self.steam_paths
            .remove_steam_folder_at_index(index as usize)
    }
    ///This sets the paths of the individual game config files, for every game in every profile.
    ///It is important that the correct paths, using this function, are set before accessing the files, or accessing files will produce errors.
    pub fn set_paths(&mut self) -> Result<(), io::Error> {
        for profile in self.profiles.iter_mut() {
            profile.set_all_paths(&self.steam_paths, self.platform.clone())?;
        }
        Ok(())
    }
    ///This sets the sensitivity value of a game, in a specific profile to the value in their local configuration files.
    pub fn fs_read_game_sens_at_index(
        &mut self,
        game: SupportedGames,
        index: i32,
    ) -> Result<(), io::Error> {
        self.profiles[index as usize].fs_read_game_sens(game)?;
        Ok(())
    }
    ///This sets all of the sensitivity values in a profile to the value in their configuration files.
    pub fn fs_read_all_game_sens_at_index(&mut self, index: i32) -> Result<(), io::Error> {
        self.profiles[index as usize].fs_read_all_game_sens()?;
        Ok(())
    }
    ///This saves the vector of profiles to json so that the profiles can be retrieved from storage later.
    pub fn save_json(&self) -> Result<(), io::Error> {
        let homepath: String = dirs::config_dir().unwrap().to_str().unwrap().to_string();

        if !(Path::new(&(homepath.clone() + "/sens/profiles.json")).exists()) {
            if !(Path::new(&(homepath.clone())).exists()) {
                fs::create_dir(homepath.clone()).unwrap();
            }
            if !(Path::new(&(homepath.clone() + "/sens")).exists()) {
                fs::create_dir(homepath.clone() + "/sens").unwrap();
            }
            File::create(homepath.clone() + "/sens/profiles.json")?.write_all(b"")?;
        } else {
            fs::remove_file(homepath.clone() + "/sens/profiles.json")?;
        }
        File::create(homepath + "/sens/profiles.json")?
            .write(serde_json::to_string(&self).unwrap().as_bytes())?;
        Ok(())
    }
    ///This retrieves the sensitivity profiles from storage so that they can be used
    pub fn fs_load_profiles() -> Result<Self, io::Error> {
        let homepath: String = dirs::config_dir().unwrap().to_str().unwrap().to_string();

        if !(Path::new(&(homepath.clone() + "/sens/profiles.json")).exists()) {
            return Ok(Self::new());
        }

        let file = File::open(homepath + "/sens/profiles.json");
        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents)?;
        let tmp = serde_json::from_str(&contents).unwrap();
        Ok(tmp)
    }

    ///This makes a string that lists the profiles and steam folders in the Profiles struct
    pub fn to_string(&self, steam: bool) -> String {
        let mut return_string: String = "".to_string();

        if steam {
            for i in 0..self.steam_paths.len() {
                return_string = return_string
                    + &self.steam_paths.get_steam_folder_at_pointer(i as usize)
                    + "\n";
            }
        } else {
            for i in 0..self.profiles.len() {
                return_string = return_string + &self.profiles[i].get_name() + "\n";
            }
        }

        return_string
    }
    ///This adds a steam folder with a specific path to the struct
    pub fn append_steam_folder(&mut self, path: String) {
        self.steam_paths.add_steam_folder(path);
    }
    //This to_string's an entire profile
    pub fn show_profile(&self, index: i32) -> String {
        self.profiles[index as usize].to_string()
    }
    //This changes the name of a profile at a specifc index
    pub fn change_name_at_index(&mut self, index: i32, name: String) {
        self.profiles[index as usize].set_name(name);
    }
    //This verifies that the profiles.json(where the profile object is permenantly stored) file exists
    pub fn verify_existence() -> Result<(), io::Error> {
        let homepath: String = dirs::config_dir().unwrap().to_str().unwrap().to_string();

        if !(Path::new(&(homepath.clone() + "/sens/profiles.json")).exists()) {
            Profiles::new().save_json()?;
        }
        Ok(())
    }
}
