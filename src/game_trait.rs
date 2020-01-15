use crate::platform::Platform;
use crate::steam_folder::SteamFolders;
use std::io;

pub trait Game {
    //default constructor
    fn new() -> Self;

    //set sens within the object
    fn get_sens(&self) -> f64;
    fn set_sens(&mut self, value: f64);

    //convert to and from csgo sens, the standard sens unit
    fn convert_to_csgo(value: f64) -> f64;
    fn convert_from_csgo(value: f64) -> f64;

    //write to the game's config file in the fs and read the
    fn fs_read(&self) -> Result<f64, io::Error>;
    fn fs_write(self) -> Result<(), io::Error>;

    //deal with the file path of the game config file
    fn get_path(&self) -> String;
    fn set_path(
        &mut self,
        steam_path: SteamFolders,
        platform_value: Platform,
    ) -> Result<(), io::Error>;
}
