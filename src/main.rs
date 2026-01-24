mod config;
mod mod_checker;
use std::path::{Path, PathBuf};

use config::Config;

use crate::mod_checker::file_reader;

fn main()  {
    let config = Config::init();
    let mod_info = file_reader( &PathBuf::from("./test_folder/mods/bluemap.jar")).expect("something");
    println!("{:?}", mod_info);

    
}
