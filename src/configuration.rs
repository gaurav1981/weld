extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use weld;

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Configuration {
    pub server: Server,
    pub database: Database,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            database: Database {
                path: "db.json".to_string(),
                default_pk: "id".to_string(),
            },
            server: Server {
                port: 8080,
                host: "127.0.0.1".to_string(),
            },
        }
    }
    pub fn load(&mut self, path: &str) {
        info!(weld::ROOT_LOGGER,
              "Configuration - Reading Path: {:?}",
              &path);
        let mut file = File::open(path)
            .expect("Configuration - Error Can't read provided configuration. Terminating...");
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(size) => {
                if size == 0 {
                    panic!("Configuration - Error Empty File Terminating...");
                }
                let config: Configuration = serde_json::from_str(&contents)
                    .expect("Configuration - Error Invalid JSON format. Terminating...");
                info!(weld::ROOT_LOGGER, "Configutation - Ok");
                debug!(weld::ROOT_LOGGER, "{:?}", &config);
                self.server = config.server;
                self.database = config.database;
            }
            Err(e) => {
                error!(weld::ROOT_LOGGER, "Configuration - Error : {}", e);
                panic!("Configuration - Error Terminating...");
            }
        }


    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Server {
    pub host: String,
    pub port: i16,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Database {
    pub path: String,
    pub default_pk: String,
}
