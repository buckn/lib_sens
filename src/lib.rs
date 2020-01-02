mod game_trait;
mod games_enum;
mod platform;
mod profile;
pub mod profile_manager;
mod steam_folder;

#[cfg(test)]
mod tests {
    use crate::games_enum::SupportedGames;
    use crate::profile_manager::Profiles;
    use crate::steam_folder::SteamFolders;

    #[test]
    fn prof_man_test() {
        let mut x = Profiles::new();
        x.add_profile();
        x.set_platform();
        x.add_steam_folder("/home/test/.steam/steam/".to_string());
        x.set_paths();
        x.equalize_profile_at_index(SupportedGames::CSGO, 0);
        x.save_json();
    }
    #[test]
    fn steam_folder_test() {
        let x = SteamFolders::new();
    }

}
