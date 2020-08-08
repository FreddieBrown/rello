use std::{fs::File, io::prelude::*, fmt};
use clap::{Arg, App};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    counter: u16,
    columns: Vec<Column>
}

impl Board {
    pub fn new() -> Board{
        Board{counter: 0, columns: vec![]}
    }

    pub fn create_column(&mut self, title: String) {
        self.columns.push(Column::new(title));
    }
    
    pub fn remove_column(&mut self, title: String) {
        for i in 0..self.columns.len(){
            if self.columns[i].title == title {
                self.columns.remove(i);
                return
            }
        }
    }

    pub fn create_item(&mut self, item_title: String, item_body: String, col_title: String) {
        for col in self.columns.iter_mut(){
            if col.title == col_title {
                col.items.push(Item::new(self.counter, item_title, item_body));
                self.counter += 1;
                return
            }
        }
    }
    
    pub fn remove_item(&mut self, id: u16) {
        for col in self.columns.iter_mut(){
            for i in 0..col.items.len() {
                if col.items[i].id == id {
                    col.items.remove(i);
                    return
                }
            }
        }
    }

    pub fn edit_item_title(&mut self, id: u16, title: String){
        for col in self.columns.iter_mut(){
            for itm in col.items.iter_mut() {
                if itm.id == id {
                    itm.title = title;
                    return
                }
            }
        }
    }

    pub fn edit_item_body(&mut self, id: u16, body: String){
        for col in self.columns.iter_mut(){
            for itm in col.items.iter_mut() {
                if itm.id == id {
                    itm.body = body;
                    return
                }
            }
        }
    }
    
    pub fn move_item(&mut self, id: u16, col_title: String){
        let mut target_col: Option<&mut Column> = None;
        let mut target_itm: Option<Item> = None;
        for col in self.columns.iter_mut() {
            if col.title == col_title {
                target_col = Some(col);
                continue
            }
            for i in 0..col.items.len() {
                if col.items[i].id == id {
                    target_itm = Some(col.items.remove(i));
                    continue
                }
            }
        }

        match target_col{
            Some(col) => match target_itm {
                Some(itm) => col.items.push(itm),
                None => println!("That item doesn't exist")
            },
            None => println!("That column doesn't exist"),
        }
    }

    fn column_exists(&self, title: String) -> bool {

        for col in self.columns.iter() {
            if col.title == title {
                return true
            }
        }
        false
    }

    fn item_exists(&self, id: u16) -> Option<&String> {
        for col in self.columns.iter() {
            for itm in col.items.iter() {
                if itm.id == id {
                    return Some(&col.title)
                }
            }
        }
        None
    }


}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let whole: Vec<_> = self.columns.iter().map(|x| x.to_string()).collect();
        write!(f, "Board: \n\t{}", whole.join("\n\t"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Column {
    title: String,
    items: Vec<Item>
}

impl Column {
    pub fn new(title: String) -> Column {
        Column{title, items: vec![]}
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let whole: Vec<_> = self.items.iter().map(|x| x.to_string()).collect();
        write!(f, "{}: \n\t\t{}", self.title, whole.join("\n\t\t"))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: u16,
    title: String,
    body: String
}

impl Item {
    pub fn new(id: u16, title: String, body: String) -> Item {
        Item{id: id, title, body}
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(ID: {}) Title: {}, Body: {}", self.id, self.title, self.body)
    }
}

fn main() {
    let matches = App::new("Rello")
        .version("0.0.1")
        .author("Freddie Brown")
        .about("Command Line PM Board")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List Current Board"))
        .arg(Arg::with_name("add_column")
            .help("Add Column to Board")
            .long("add_column")
            .takes_value(true))
        .arg(Arg::with_name("remove_column")
            .help("Remove Column From Board")
            .long("remove_column")
            .takes_value(true))
        .arg(Arg::with_name("edit_item")
            .long("edit_item")
            .help("Edit Item Data"))
        .arg(Arg::with_name("add_item")
            .long("add_item")
            .help("Add Item to Column")
            .takes_value(true))
        .arg(Arg::with_name("remove_item")
            .long("remove_item")
            .help("Remove Item From Column")
            .takes_value(true))
        .arg(Arg::with_name("move_item")
            .long("move_item")
            .help("Move Item To Different Column")
            .takes_value(true))
        .get_matches();
    let mut config_toml = String::from("");
    let file_path = "board.toml";
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            File::create(&file_path).expect("Cannot create new file")
        }
    };
    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let mut config: Board = toml::from_str(&config_toml).unwrap();
    
    if matches.is_present("list"){
        println!("{}", config.to_string());
    }

    if matches.is_present("add_column"){
        match matches.value_of("add_column"){
            Some(name) => config.create_column(String::from(name)),
            None => ()
        };

        println!("Column Added!");
    }

    if matches.is_present("remove_column"){
        match matches.value_of("remove_column"){
            Some(name) => config.remove_column(String::from(name)),
            None => ()
        };

        println!("Column Removed!");
    }

    if matches.is_present("add_item"){
        println!("Adding Item")
    }

    if matches.is_present("remove_item"){
        match matches.value_of("remove_item"){
            Some(id) => config.remove_item(id.parse::<u16>().unwrap()),
            None => ()
        };

        println!("Item Removed!")
    }

    if matches.is_present("move_item"){
        println!("Moving Item")
    }

    if matches.is_present("edit_item"){
        println!("Editing Item")
    }
    close(&config, file_path);
}

fn close(board: &Board, file: &str) {
    let toml = toml::to_string(board).unwrap();
    std::fs::write(file, toml).expect("Unable to write file");
}

#[cfg(test)]
mod tests;