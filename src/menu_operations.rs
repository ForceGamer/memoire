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
    println!("\nSelect>\n1");
}