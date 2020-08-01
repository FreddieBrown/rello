use std::{fs::File, io::prelude::*};
use clap::{Arg, App};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Board {
    columns: Vec<Column>
}

impl Board {
    pub fn new() -> Board{
        Board{columns: vec![]}
    }
}

#[derive(Deserialize, Debug)]
struct Column {
    title: String,
    items: Vec<Item>
}

#[derive(Deserialize, Debug)]
struct Item {
    title: String,
    body: String
}

fn main() {
    let matches = App::new("Rello")
        .version("0.0.1")
        .author("Freddie Brown")
        .about("Command Line PM Board")
        .get_matches();
    let config = "board.toml";
    let mut config_toml = String::from("");
    let mut file = match File::open(&config) {
        Ok(file) => file,
        Err(_) => {
            panic!("Could not find config file, using default!");
        }
    };
    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let config: Board = toml::from_str(&config_toml).unwrap();

    for cols in config.columns {
        println!("{}:", cols.title);
        for item in cols.items {
            println!("{}: {}",item.title, item.body);
        }
        println!("\n");
    }
}
