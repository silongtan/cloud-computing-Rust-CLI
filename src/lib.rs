use std::{fs};
use walkdir::WalkDir;
// use glob::glob;

pub fn list_files() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

pub fn dir_size() {
    let total_size = WalkDir::new(".")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}

pub fn terminal() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "list" => list_files(),
            "size" => dir_size(),
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }
}