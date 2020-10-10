#[derive(Debug, Copy, Clone)]
pub enum SupportedGames {
    CSGO = 0,
    PORTAL2 = 1,
    TF2 = 2,
    DS = 3,
}

impl SupportedGames {
    pub fn from_str(game_string: &str) -> Self {
        match game_string.to_lowercase().as_str() {
            "csgo" => SupportedGames::CSGO,
            "portal2" => SupportedGames::PORTAL2,
            "tf2" => SupportedGames::TF2,
            "ds" => SupportedGames::DS,
            _ => {
                eprintln!("No game matched {}!", game_string);
                SupportedGames::CSGO
            }
        }
    }

    pub fn display(&self) -> String {
        match self {
            SupportedGames::CSGO => "CS:GO".to_string(),
            SupportedGames::PORTAL2 => "Portal 2".to_string(),
            SupportedGames::TF2 => "Team Fortress 2".to_string(),
            SupportedGames::DS => "Dead Space".to_string(),
        }
    }
}
