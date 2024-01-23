use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use log::{error, info};

#[derive(Deserialize)]
pub struct Configuration {
    pub environments: Vec<Entry>
}

#[derive(Deserialize)]
pub struct Entry {
    pub env_name: String,
    pub ip: String,
    pub user: String,
    pub password: String,
}

impl Configuration {
    pub fn init(&mut self) -> Self{
        let path = Path::new("C:/Program Files/Console_Workstation/configuration.conk");
        let mut file = match File::open(path) {
            Err(why) => {
                panic!("Couldn't open configuration {:?}: {}", path, why)
            },
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => error!("Couldn't read from file {:?}: {}", path, why),
            Ok(_) => info!("Settings file content: {}", s)
        };
        serde_json::from_str(&*s).unwrap()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            environments: vec![]
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
       Entry {
           env_name: "env_name".to_string(),
           ip: "127.0.0.1".to_string(),
           user: "user".to_string(),
           password: "password".to_string(),
       }
    }
}