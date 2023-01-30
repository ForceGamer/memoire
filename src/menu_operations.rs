use std::io;
use crate::colors::*;
use crate::repeat_game;

pub struct Menu {
    pub(crate) title: String,
    pub(crate) multi: bool, // If multiple choices can be selected
    pub(crate) choices: Vec<String>
}

pub fn make_menu(title:String, multi:bool, choices: Vec<String>) -> Menu {
    Menu {
        title,
        multi,
        choices,
    }
}
pub fn open_menu(menu:&Menu) {
    loop {
        //Title
        println!("{}{}{}\n", blue(), menu.title, endcolor());

        //Menu
        for (i, el) in menu.choices.iter().enumerate() {
            println!("{} {el}", i + 1);
        }
        println!("\nType 'q' to Quit");
        let input = get_input("\nSelect:");

        // Break if user asks to Quit
        if input.starts_with("q") {break;}

        let input = parse_int(input);
        match input {
            1 => { repeat_game::start() }
            _ => { println!("{}Invalid selection{}", red(), endcolor()) }
        }
    }
}

pub fn get_input(prompt:&str) -> String {
    if prompt.len() > 0 {
        println!("{prompt}");
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Whoah there!"); //TODO: Better error handling
    return input;
}

//Parses and converts String to u8 cleanly
pub fn parse_int(string:String) -> u8 {
    return string.trim()
        .parse()
        .expect("Not a number!"); //TODO: Better error handling
}