#![allow(unused)]
use std::env;

use rust_cli::{list_files, recently_modified, dir_size};

// fn terminal() {
//     loop {
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         let input = input.trim();

//         match input {
//             "list" => list_files(),
//             "recent" => recently_modified(),
//             "size" => dir_size(),
//             "exit" => break,
//             _ => println!("Invalid command"),
//         }
//     }
// }

fn main() {
    // terminal();
    list_files();
    recently_modified();
    dir_size();
}