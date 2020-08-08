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
    config.create_column(String::from("Test"));
    config.create_item(String::from("TestItemTitle1"), String::from("TestItemBody1"), String::from("Test"));
    config.create_item(String::from("TestItemTitle2"), String::from("TestItemBody2"), String::from("Test"));

    println!("{}", config.to_string());

    config.move_item(config.counter-1, String::from("ToDo"));

    println!("{}", config.to_string());

    config.remove_item(config.counter-1);

    println!("{}", config.to_string());

    config.remove_column(String::from("Test"));
    

    close(&config, file_path);
}

fn close(board: &Board, file: &str) {
    let toml = toml::to_string(board).unwrap();
    std::fs::write(file, toml).expect("Unable to write file");
}

#[cfg(test)]
mod tests;