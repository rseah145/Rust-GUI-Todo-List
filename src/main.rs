// hide console window on Windows in Release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod todo_list;

use todo_list::Todos;
use eframe::{run_native, NativeOptions, Theme};
//use egui::viewport::{ViewportBuilder, IconData};
//use image::io::Reader as image;
//use std::path::MAIN_SEPARATOR;

// todo item module
// using path attr to read a file (not recommended)
// old code that is no longer applicable
//#[path = "../lib/todo_item.rs"]
//mod todo_item;

fn main() {
    /* Need to figure out why it does not work as intended
    let icon_image = image::open("app_icon".to_owned() + &MAIN_SEPARATOR.to_string() + "to-do-list.png")
        .expect("test")
        .with_guessed_format()
        .expect("test")
        .decode()
        .unwrap();
    let width = icon_image.width();
    let height = icon_image.height();
    let icon_rgba8 = icon_image.into_rgb8().to_vec();
    let icon = IconData {
        rgba: icon_rgba8,
        width,
        height,
    };
    */

    let window_option = NativeOptions {
        follow_system_theme: false,
        default_theme: Theme::Dark,
        //viewport: ViewportBuilder::default()
        //    .with_icon(icon),
        ..Default::default()
    };
    let _ = run_native("Rust egui Todo-List", window_option, Box::new(|cc| Box::new(Todos::new(cc))));
}
