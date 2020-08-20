#![allow(dead_code, irrefutable_let_patterns)]
use clap::{App, Arg};
mod board;
use board::*;

fn main() {
    let matches = App::new("Rello")
        .version("0.0.1")
        .author("Freddie Brown")
        .about("Command Line PM Board")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List Current Board"),
        )
        .arg(
            Arg::with_name("add_column")
                .help("Add Column to Board. Pass Column Name.")
                .long("add_column")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("remove_column")
                .help("Remove Column From Board. Pass Column Name.")
                .long("remove_column")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("edit_item")
                .long("edit_item")
                .help("Edit Item Data. Pass Item ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("add_item")
                .long("add_item")
                .help("Add Item to Column"),
        )
        .arg(
            Arg::with_name("remove_item")
                .long("remove_item")
                .help("Remove Item From Column. Pass Item ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("move_item")
                .long("move_item")
                .help("Move Item To Different Column"),
        )
        .arg(
            Arg::with_name("shell")
                .long("shell")
                .short("s")
                .help("Interactive shell allowing multiple interactions with board"),
        )
        .get_matches();
    let file_path = "./board.toml";

    let mut board: Board = match std::fs::read_to_string(file_path) {
        Ok(s) => match toml::from_str(&s) {
            Ok(b) => b,
            Err(_) => Board::new(),
        },
        Err(_) => Board::new(),
    };

    if matches.is_present("shell") {
        println!("SHELL");
        shell(&mut board);
    } else {
        process_info(&matches, &mut board);
    }

    close(&board, file_path);
}

fn shell(board: &mut Board) {
    while let input = Board::read_text(String::from("rello")) {
        let ci_input = input.split(" ").collect::<Vec<&str>>();
        if ci_input[0] == "exit" {
            return;
        }

        if ci_input[0] == "list" {
            println!("{}", board.to_string());
        }

        if ci_input[0] == "add_column" {
            if ci_input.len() > 1 {
                board.create_column(String::from(ci_input[1]));
            }

            println!("Column Added!");
        }

        if ci_input[0] == "remove_column" {
            if ci_input.len() > 1 {
                board.remove_column(String::from(ci_input[1]))
            }

            println!("Column Removed!");
        }

        if ci_input[0] == "add_item" {
            let title = Board::read_text(String::from("Enter Title"));
            let body = Board::read_text(String::from("Enter Body"));
            let assignee = Board::read_text(String::from("Enter Assignee (Press Enter if none)"));
            let col_title = Board::read_text(String::from("Enter Column"));

            match &assignee[..] {
                "" => board.create_item(title, body, None, col_title),
                _ => board.create_item(title, body, Some(assignee), col_title),
            };
            println!("Item Added!");
        }

        if ci_input[0] == "remove_item" {
            if ci_input.len() > 1 {
                board.remove_item(ci_input[1].parse::<u16>().unwrap());
            }

            println!("Item Removed!")
        }

        if ci_input[0] == "move_item" {
            let id = Board::read_text(String::from("Enter Item Id"))
                .parse::<u16>()
                .unwrap();
            let col_title = Board::read_text(String::from("Enter Column"));
            board.move_item(id, col_title);
            println!("Moved Item")
        }

        if ci_input[0] == "edit_item" {
            let blank = String::from("");
            let title = Board::read_text(String::from("New Title (Press Enter If No Change)"));
            if title != blank {
                board.edit_item_title(ci_input[1].parse::<u16>().unwrap(), title);
            } else {
                println!("Blank Title")
            }
            let body = Board::read_text(String::from("New Body (Press Enter If No Change)"));
            if body != blank {
                board.edit_item_body(ci_input[1].parse::<u16>().unwrap(), body);
            } else {
                println!("Blank Body")
            }

            println!("Item Edited!");
        }
    }

    println!("Shell Closed");
}

fn process_info(matches: &clap::ArgMatches, board: &mut Board) {
    if matches.is_present("list") {
        println!("{}", board.to_string());
    }

    if matches.is_present("add_column") {
        match matches.value_of("add_column") {
            Some(name) => board.create_column(String::from(name)),
            None => (),
        };

        println!("Column Added!");
    }

    if matches.is_present("remove_column") {
        match matches.value_of("remove_column") {
            Some(name) => board.remove_column(String::from(name)),
            None => (),
        };

        println!("Column Removed!");
    }

    if matches.is_present("add_item") {
        let title = Board::read_text(String::from("Enter Title"));
        let body = Board::read_text(String::from("Enter Body"));
        let assignee = Board::read_text(String::from("Enter Assignee (Press Enter if none)"));
        let col_title = Board::read_text(String::from("Enter Column"));

        match &assignee[..] {
            "" => board.create_item(title, body, None, col_title),
            _ => board.create_item(title, body, Some(assignee), col_title),
        };
        println!("Item Added!");
    }

    if matches.is_present("remove_item") {
        match matches.value_of("remove_item") {
            Some(id) => board.remove_item(id.parse::<u16>().unwrap()),
            None => (),
        };

        println!("Item Removed!")
    }

    if matches.is_present("move_item") {
        let id = Board::read_text(String::from("Enter Item Id"))
            .parse::<u16>()
            .unwrap();
        let col_title = Board::read_text(String::from("Enter Column"));
        board.move_item(id, col_title);
        println!("Moved Item")
    }

    if matches.is_present("edit_item") {
        let blank = String::from("");
        let title = Board::read_text(String::from("New Title (Press Enter If No Change)"));
        if title != blank {
            board.edit_item_title(
                matches
                    .value_of("edit_item")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap(),
                title,
            );
        } else {
            println!("Blank Title")
        }
        let body = Board::read_text(String::from("New Body (Press Enter If No Change)"));
        if body != blank {
            board.edit_item_body(
                matches
                    .value_of("edit_item")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap(),
                body,
            );
        } else {
            println!("Blank Body")
        }

        println!("Item Edited!");
    }
}

fn close(board: &Board, file: &str) {
    let toml = toml::to_string(board).unwrap();
    std::fs::write(file, toml).expect("Unable to write file");
}

#[cfg(test)]
mod tests;
