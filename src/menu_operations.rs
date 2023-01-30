use std::io;
use crate::colors::*;

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
    println!("{}{}{}", blue(), menu.title, endcolor());
    for (i, el) in menu.choices.iter().enumerate() {
        println!("{} {el}", i+1);
    }
    let input = get_input("Select:");
    let input:u8 = input.trim()
        .parse()
        .expect("Not a number!"); //TODO: Better error handling
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