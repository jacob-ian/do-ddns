use std::{fs, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub schedule: String,
    pub domain: String,
    pub record_name: String,
    pub do_token: String,
}

impl Config {
    pub fn from_toml() -> io::Result<Config> {
        let path = Self::find_config_path().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find config file",
        ))?;
        println!("Config file found: {}", &path);
        let config = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&config).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid config: {}", e.to_string()),
            )
        })?;

        Ok(config)
    }

    fn find_config_path() -> Option<String> {
        let locations: Vec<String> = vec![
            String::from("./config.toml"),
            String::from("/etc/do-ddns/config.toml"),
        ];
        let mut i = 0;
        loop {
            if i == locations.len() {
                break None;
            }
            let location = locations.get(i).expect("Config path index out of bounds");
            if Path::new(&location).is_file() {
                break Some(location.to_owned());
            }
            i = i + 1;
        }
    }
}
