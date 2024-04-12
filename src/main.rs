// hide console window on Windows in Release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod todo_list;

use todo_list::Todos;
use eframe::{run_native, NativeOptions, Theme};

// todo item module
// using path attr to read a file (not recommended)
// old code that is no longer applicable
//#[path = "../lib/todo_item.rs"]
//mod todo_item;

fn main() {
    //let test_todo = todo_item::TodoItemEntry { item_text: String::from("test"), item_desc: String::from("test2"), is_done: true };
    //println!("{:?}", test_todo);
    let mut win_option = NativeOptions::default();
    win_option.follow_system_theme = false;
    win_option.default_theme = Theme::Dark;
    let _ = run_native("Rust egui Todo-List", win_option, Box::new(|cc| Box::new(Todos::new(cc))));
}
