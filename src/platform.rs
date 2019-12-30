use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Platform {
    Linux = 0,
    Windows = 1,
    Mac = 2,
    Other = 3,
}

impl Platform {
    pub fn new() -> Platform {
        if cfg!(windows) {
            Platform::Windows
        } else if cfg!(unix) {
            if Path::new("/Library/Application Support").exists() {
                Platform::Mac
            } else {
                Platform::Linux
            }
        } else {
            Platform::Other
        }
    }
}
