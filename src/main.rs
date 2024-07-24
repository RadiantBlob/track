use std::env;
use std::fs::{File, OpenOptions};
use std::path::{PathBuf};
use std::io::{BufReader, BufRead};
use dirs::home_dir;

fn get_file() -> Result<File, std::io::Error> {
    let home = home_dir().expect("Couldnt find HOME");
    let file_path: PathBuf = home.join("track_list.t");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)?;

    Ok(file)
}

fn get_paths(file: &File) -> Vec<PathBuf> {
    let reader = BufReader::new(file);

    let mut paths = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let path = PathBuf::from(line);
        paths.push(path)
    }

    paths
}

fn handle_add(file: &mut File) {
    println!("ADD was found");

    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let mut file: File = match get_file() {
    //     Err(e) => {
    //         println!("An Error occured while opening ~/track_list.t:\n{}", e);
    //         panic!("Error opening file")
    //     },
    //     Ok(file) => file,
    // };
    let mut file: File = get_file().unwrap();

    let paths = get_paths(&file);

    println!("Args: {:?}", args);
    println!("Found Paths are: {:?}", paths);

    if args.len() == 3 {
        match args[1].as_str() {
            "add" => handle_add(&mut file),
            _ => println!("'{}' was found", args[1])
        }
    }
}
