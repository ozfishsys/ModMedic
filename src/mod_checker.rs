
use std::{error::Error, fs, io::{self ,Read}, path::{Path, PathBuf}};

use crate::config::Config;

// grabs the file path of all the mods in the mods folder and save to a vec
pub fn get_file_path(config: &mut Config) -> Vec<PathBuf>{
    let mut mods: Vec<PathBuf> = Vec::new();

    let mut path = PathBuf::from(config.file_path.as_str());

    let mut mods_path = String::new();

    // when users mods folder is not the usual mods folder
    // ask user and updates the file path
    loop {
        if path.exists() || path.is_dir() {
            break;
        } 
        println!("what is the relative path to your mods folder?");
        mods_path.clear();
        io::stdin().read_line(&mut mods_path).expect("failed to read line");

        let input = mods_path.trim();
        path = PathBuf::from(input);
        config.file_path = input.to_string();
    }

    for entry in fs::read_dir(path).expect("problem ") {
        let entry = entry.expect("problem ");
        let path = entry.path();

        if path.is_file() {
            mods.push(path);
        }
    }
    mods
}

// a jar file is a zip file just different format
// opens the jar file looks inside for the file
pub fn file_reader(rpath: &Path) -> Result<String, Box<dyn Error>> {
    // tries to open the zip file
    let zipfile = match std::fs::File::open(&rpath) {
        Ok(f) => f,
        Err(e) => {
            println!("Error: failed to open {:?}: {e}", rpath.display());
            return Err(e.into());
        }
    };

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    // tries to read the need file
    let mut file = match archive.by_name("fabric.mod.json") {
        Ok(file) => file,
        Err(e) => {
            println!("File not found");
            return Err(e.into());
        }
    };
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{contents}");

    Ok(contents)
}