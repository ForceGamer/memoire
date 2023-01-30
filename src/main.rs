use crate::menu_operations::open_menu;
use crate::menus::main_menu;

mod repeat_game;
mod menu_operations;
mod colors;
mod menus;

fn main() {
    open_menu(&main_menu());
    repeat_game::start();
}
