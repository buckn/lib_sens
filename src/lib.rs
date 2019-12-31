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
        Profiles::check_config_dir();
        let mut x = Profiles::new();
        x.add_profile();
        x.add_profile();
        x.remove_profile(1);
        x.set_platform();
        x.add_steam_folder("/".to_string());
        x.add_steam_folder("/".to_string());
        x.remove_steam_folder_at_index(0);
        println!("Steam Folder: {:?}", x.get_steam_folder_at_pointer(0));
        x.set_paths();
        x.equalize_profile_at_index(SupportedGames::CSGO, 0);
        x.fs_save_profiles();
        let y = Profiles::fs_load_profiles();
        x = Profiles::from_json_string(x.to_json_string());
        assert!(x.to_json_string() == y.to_json_string());

    }
}
