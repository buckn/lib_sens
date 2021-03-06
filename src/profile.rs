///This is an implementation of the Game trait for CSGO.
mod supp_csgo;
///This is an implementation of the Game trait for Dead Space.
mod supp_ds;
///This is an implementation of the Game trait for Portal 2.
mod supp_portal2;
///This is an implementation of the Game trait for Team Fortress 2.
mod supp_tf2;

use crate::game_trait::Game;
use crate::games_enum::SupportedGames;
use crate::platform::*;
use crate::profile::supp_csgo::CSGO;
use crate::profile::supp_ds::DS;
use crate::profile::supp_portal2::PORTAL2;
use crate::profile::supp_tf2::TF2;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
pub struct SensProfile {
    game_vec: Vec<Box<dyn Game>>,
    name: String,
}

impl SensProfile {
    ///Creates a new sens profile
    pub fn new() -> Self {
        Self {
            game_vec: vec![
                Box::new(CSGO::new()),
                Box::new(PORTAL2::new()),
                Box::new(TF2::new()),
                Box::new(DS::new()),
            ],
            name: "untitled_profile".to_string(),
        }
    }
    ///Makes all of the values in a profile equivalent to one of the values of a game in the profile
    pub fn equalize(&mut self, game: SupportedGames) {
        let set_sens = self.game_vec[game as usize].convert_self_to_csgo();
        for item in &mut self.game_vec {
            item.set_sens_from_csgo_sens(set_sens);
        }
    }
    ///This reads all of the sensitivities of the games from their config files and stores them into the Profile object
    pub fn fs_read_all_game_sens(&mut self) -> Result<(), io::Error> {
        for item in &mut self.game_vec {
            item.set_sens(item.fs_read()?);
        }
        Ok(())
    }
    ///This sets the sensitivity of a game to the value in its respective config file
    pub fn fs_read_game_sens(&mut self, game: SupportedGames) -> Result<(), io::Error> {
        self.game_vec[game as usize].set_sens_to_fs_value()?;
        Ok(())
    }
    ///This sets the value of a game's sensitivity to a specific number
    pub fn set_game_sens(&mut self, game: SupportedGames, sens: f64) {
        self.game_vec[game as usize].set_sens(sens);
    }
    ///
    pub fn save_all_to_configs(&self) -> Result<(), io::Error> {
        for item in &self.game_vec {
            item.fs_write()?;
        }
        Ok(())
    }
    pub fn set_all_paths(
        &mut self,
        steam_paths: &SteamFolders,
        platform_value: Platform,
    ) -> Result<(), io::Error> {
        for item in &mut self.game_vec {
            item.set_path(steam_paths, platform_value.clone())?;
        }
        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn to_string(&self) -> String {
        let mut string: String = "Game Sensitivities: \n\n".to_string();
        for item in &self.game_vec {
            string = string + &item.to_string();
        }
        string
    }
}
