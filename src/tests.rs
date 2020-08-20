use super::*;
use std::{fs::File, io::prelude::*};

fn setup(file_path: &str) -> Board {
    let mut config_toml = String::from("");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => File::create(&file_path).expect("Cannot create new file"),
    };
    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    toml::from_str(&config_toml).unwrap()
}

#[test]
fn test_sanity() {
    assert_eq!(1, 1);
}

#[test]
fn test_create_remove_column() {
    let file_path = "test_board.toml";

    let mut config: Board = setup(file_path);

    config.create_column(String::from("Test"));

    assert!(config.column_exists(String::from("Test")));

    config.remove_column(String::from("Test"));

    assert!(!config.column_exists(String::from("Test")));
}

#[test]
fn test_create_remove_item() {
    let file_path = "test_board.toml";

    let mut config: Board = setup(file_path);

    config.create_item(
        String::from("TestItemTitle1"),
        String::from("TestItemBody1"),
        None,
        String::from("ToDo"),
    );

    match config.item_exists(config.counter - 1) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    config.remove_item(config.counter - 1);
    match config.item_exists(config.counter - 1) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
}

#[test]
fn test_create_remove_item_with_assignee() {
    let file_path = "test_board.toml";

    let mut config: Board = setup(file_path);

    config.create_item(
        String::from("TestItemTitle1"),
        String::from("TestItemBody1"),
        Some(String::from("TestAssignee1")),
        String::from("ToDo"),
    );

    match config.item_exists(config.counter - 1) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    config.remove_item(config.counter - 1);
    match config.item_exists(config.counter - 1) {
        Some(_) => assert!(false),
        None => assert!(true),
    };
}

#[test]
fn test_move_item() {
    let file_path = "test_board.toml";

    let mut config: Board = setup(file_path);

    config.create_item(
        String::from("TestItemTitle1"),
        String::from("TestItemBody1"),
        None,
        String::from("ToDo"),
    );

    config.move_item(config.counter - 1, String::from("Doing"));
    match config.item_exists(config.counter - 1) {
        Some(col) => assert_eq!(col, &String::from("Doing")),
        None => assert!(false),
    };

    config.remove_item(config.counter - 1);
}
