mod game_trait;
mod games_enum;
mod platform;
mod profile;
mod profile_manager;
mod steam_folder;

#[cfg(test)]
mod tests {
    use crate::games_enum::SupportedGames;
    use crate::profile_manager::Profiles;

    #[test]
    fn prof_man() {
        let mut x = Profiles::new();
        x.add_profile();
        x.set_platform();
        x.add_steam_folder("/Users/nathanielbuck/Library/Application Support/Steam/".to_string());
        x.set_paths();
        x.fs_read_game_sens_at_index(SupportedGames::CSGO, 0);
        x.equalize_profile_at_index(SupportedGames::CSGO);
        x.switch_profile(0);
        x.save_json();
        println!("{:?}", Profiles::load_json());
    }
}
