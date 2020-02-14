use crate::platform::Platform;
use crate::steam_folder::SteamFolders;
use std::io;

#[typetag::serde(tag = "type")]
pub trait Game {
    //set sens within the object
    fn get_sens(&self) -> f64;
    fn set_sens(&mut self, value: f64);
    fn set_sens_to_fs_value(&mut self) -> Result<(), io::Error>;
    fn set_sens_from_csgo_sens(&mut self, value: f64);

    //convert to and from csgo sens, the standard sens unit
    fn convert_self_to_csgo(&self) -> f64;

    //write to the game's config file in the fs and read the
    fn fs_read(&self) -> Result<f64, io::Error>;
    fn fs_write(&self) -> Result<(), io::Error>;

    //deal with the file path of the game config file
    fn get_path(&self) -> String;
    fn set_path(
        &mut self,
        steam_path: &SteamFolders,
        platform_value: Platform,
    ) -> Result<(), io::Error>;

    //to string for listing games
    fn to_string(&self) -> String;
}
