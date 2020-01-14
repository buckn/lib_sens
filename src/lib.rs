///A trait that describes how to implement support for a new game.
mod game_trait;
///An enum that contains all the supported games, so that you can use match.
mod games_enum;
///A enum that represents the platform that the library is currently running on.
mod platform;
///A module for each profile that represents the sensitivities for each game in that profile.
mod profile;
///A module that manages a vector of profiles so that you are able to perform functions with all of your profiles at once.
pub mod profile_manager;
//A profile that stores the paths to various steam folders on the current system.
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
