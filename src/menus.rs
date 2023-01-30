use crate::menu_operations::Menu;

pub fn main_menu() -> Menu {
    let choices: Vec<String> = vec![
        "Repeat Recognition".to_string(),
        "Test".to_string(),
        "Extras".to_string()];
    Menu {
        title: "Welcome to Memoire".to_string(),
        multi: false,
        choices,
    }
}