#![allow(unused)]
use std::env;
use rust_cli::{terminal, list_files, dir_size};

use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Silong Tan", about = "Rust shell")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Silong Tan")]
    Run {
        #[clap(short, long)]
        name: String,
    },
}

fn greeting(name: &str) {
    println!("Welcome to the basic shell");
    println!("Type 'list' to list files in the current directory");
    println!("Type 'size' to get the size of the current directory");
    println!("Type 'exit' to exit the shell");
    println!("Hello, {}!", name);
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Run { name }) => {
            greeting(&name);
            terminal();
        }
        None => println!("No subcommand was used"),
    }
}