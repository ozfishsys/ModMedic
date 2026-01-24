use std::{fs::{self, File}, io, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_yml;

// custom date type to hold all the config information
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_path: String,
    pub mod_loader: String,
    
}

impl Config {
    
    pub fn init() -> Config {
        let mut config: Config = Config { file_path: String::from("/mods"), mod_loader: String::from("fabric") };
        let config_path = PathBuf::from("config.yaml");
        
        if config_path.exists() {
            let content = fs::read_to_string(config_path).expect("could not read config file to string");
            config = serde_yml::from_str(&content).expect("could not deserialize config file to struct");
        } else {
            let _config_file = File::create(config_path);
            println!("what mod loader are you using (fabric, forge, neoforge) ");

            let mut mod_loader = String::new();

            io::stdin().read_line(&mut mod_loader).expect("failed to read line");

            mod_loader = mod_loader.trim().to_lowercase();
            config.mod_loader = mod_loader; 
        }
        if !matches!(config.mod_loader.as_str(), "fabric" | "forge" | "neoforge") {
            println!("Sorry we do not support {} mod loader", config.mod_loader);
            loop {
                println!("what mod loader are you using (fabric, forge, neoforge) ");

                let mut mod_loader = String::new();

                io::stdin().read_line(&mut mod_loader).expect("failed to read line");

                if !matches!(mod_loader.as_str(), "fabric" | "forge" | "neoforge") {
                    println!("Sorry we do not support {} mod loader", mod_loader);
                } else {
                    config.mod_loader = mod_loader;
                    break;
                }
            }
        }
        
        config
    }
}