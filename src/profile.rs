///This is an implementation of the Game trait for CSGO.
mod supp_csgo;
///This is an implementation of the Game trait for Portal 2.
mod supp_portal2;
///This is an implementation of the Game trait for Team Fortress 2.
mod supp_tf2;

use crate::game_trait::Game;
use crate::games_enum::SupportedGames;
use crate::platform::*;
use crate::profile::supp_csgo::CSGO;
use crate::profile::supp_portal2::PORTAL2;
use crate::profile::supp_tf2::TF2;
use crate::steam_folder::SteamFolders;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GamesTuple(CSGO, PORTAL2, TF2);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensProfile {
    game_structs_tuple: GamesTuple,
    number_of_supported_games: i32,
}

impl SensProfile {
    pub fn new() -> Self {
        Self {
            game_structs_tuple: GamesTuple(CSGO::new(), PORTAL2::new(), TF2::new()),
            number_of_supported_games: 3,
        }
    }

    pub fn equalize(&mut self, game: SupportedGames) {
        let csgo_sens: f64;

        match game {
            SupportedGames::CSGO => {
                csgo_sens = CSGO::convert_to_csgo(self.game_structs_tuple.0.get_sens())
            }
            SupportedGames::PORTAL2 => {
                csgo_sens = CSGO::convert_to_csgo(self.game_structs_tuple.1.get_sens())
            }
            SupportedGames::TF2 => {
                csgo_sens = CSGO::convert_to_csgo(self.game_structs_tuple.2.get_sens())
            }
        }

        self.game_structs_tuple
            .0
            .set_sens(CSGO::convert_from_csgo(csgo_sens));
        self.game_structs_tuple
            .1
            .set_sens(PORTAL2::convert_from_csgo(csgo_sens));
        self.game_structs_tuple
            .2
            .set_sens(TF2::convert_from_csgo(csgo_sens));
    }

    pub fn fs_read_all_game_sens(&mut self) {
        self.game_structs_tuple
            .0
            .set_sens(self.game_structs_tuple.0.fs_read());
        self.game_structs_tuple
            .1
            .set_sens(self.game_structs_tuple.1.fs_read());
        self.game_structs_tuple
            .2
            .set_sens(self.game_structs_tuple.2.fs_read());
    }

    pub fn fs_read_game_sens(&mut self, game: SupportedGames) {
        match game {
            SupportedGames::CSGO => self
                .game_structs_tuple
                .0
                .set_sens(self.game_structs_tuple.0.fs_read()),
            SupportedGames::PORTAL2 => self
                .game_structs_tuple
                .1
                .set_sens(self.game_structs_tuple.1.fs_read()),
            SupportedGames::TF2 => self
                .game_structs_tuple
                .2
                .set_sens(self.game_structs_tuple.2.fs_read()),
        }
    }

    pub fn set_game_sens(&mut self, game: SupportedGames, sens: f64) {
        match game {
            SupportedGames::CSGO => self.game_structs_tuple.0.set_sens(sens),
            SupportedGames::PORTAL2 => self.game_structs_tuple.1.set_sens(sens),
            SupportedGames::TF2 => self.game_structs_tuple.2.set_sens(sens),
        }
    }
    pub fn save_all_to_configs(self) {
        self.game_structs_tuple.0.clone().fs_write();
        self.game_structs_tuple.1.clone().fs_write();
        self.game_structs_tuple.2.clone().fs_write();
    }
    pub fn set_all_paths(&mut self, steam_paths: SteamFolders, platform_value: Platform) {
        self.game_structs_tuple
            .0
            .set_path(steam_paths.clone(), platform_value.clone());
        self.game_structs_tuple
            .1
            .set_path(steam_paths.clone(), platform_value.clone());
        self.game_structs_tuple
            .2
            .set_path(steam_paths.clone(), platform_value.clone());
    }
}
