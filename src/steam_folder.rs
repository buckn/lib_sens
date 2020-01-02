use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SteamFolders {
    steam_folder_paths: Vec<String>,
}

impl SteamFolders {
    pub fn new() -> Self {
        Self {
            steam_folder_paths: Vec::new(),
        }
    }
    pub fn get_steam_folder_at_pointer(&self, x: usize) -> String {
        self.steam_folder_paths[x].to_string()
    }
    pub fn add_steam_folder(&mut self, x: String) {
        self.steam_folder_paths.push(x);
    }
    pub fn remove_steam_folder_at_index(&mut self, x: usize) {
        &self.steam_folder_paths.remove(x);
    }
    pub fn len(&self) -> usize {
        self.steam_folder_paths.len()
    }
    pub fn find_file_in_steam_paths(self, file_path: String) -> Result<String, io::Error> {
        let mut vec_index: i32 = 0;
        let mut file_string: String = "".to_string();

        for i in self.steam_folder_paths.clone() {
            if Path::new(&(i.to_owned() + &file_path)).exists() {
                file_string = self.get_steam_folder_at_pointer(vec_index as usize) + &file_path;
            }
            vec_index = vec_index + 1;
        }
        Ok(file_string)
    }
    pub fn find_file_in_steam_paths_with_id(
        self,
        config_path: String,
    ) -> Result<String, io::Error> {
        let mut vec_index: i32 = 0;
        let mut file_string: String = "".to_string();

        for i in self.steam_folder_paths.clone() {
            println!("steam user data path {:?}", &(i.clone() + "userdata/"));
            if Path::new(&(i.clone() + "userdata/")).exists() {
                let paths = fs::read_dir(i.clone() + "userdata/").unwrap();
                for path in paths {
                    let current_path_with_id =
                        path.unwrap().path().display().to_string() + &config_path;
                    print!("config path? {:?}", current_path_with_id);
                    if Path::new(&current_path_with_id).exists() {
                        println!("      does exist");
                        file_string = current_path_with_id.to_string();
                    }
                }
            }
            vec_index = vec_index + 1;
        }

        Ok(file_string)
    }
}
