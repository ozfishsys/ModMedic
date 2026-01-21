use std::{error::Error, fs, io::{self, Read}, path::{Path, PathBuf}};

fn main()  {
    let relative_path = Path::new("./test_folder/mods/bluemap.jar");
    let _ = file_reader(relative_path);
}

fn get_file_path() -> Vec<PathBuf>{
    let mut mods: Vec<PathBuf> = Vec::new();
    println!("(Ex: mods/ ) what the relative path to your mods folder: ");

    let mut relative_path = String::new();
    io::stdin().read_line(&mut relative_path).expect("failed to read line");
    let path = Path::new(relative_path.trim());
    if !path.exists() || !path.is_dir() {
        eprintln!("Directory {:?} does not exist or is not a directory", path);
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

fn file_reader(rpath: &Path) -> Result<String, Box<dyn Error>> {
    let zipfile = match std::fs::File::open(&rpath) {
        Ok(f) => f,
        Err(e) => {
            println!("Error: failed to open {:?}: {e}", rpath.display());
            return Err(e.into());
        }
    };

    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

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