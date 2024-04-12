use eframe::egui;
use egui::{Color32, Layout, Separator, FontFamily, FontId, RichText, 
TextStyle, Align, CentralPanel, menu, Direction, Align2, Pos2};
use egui_file::FileDialog;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_str, json};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf, MAIN_SEPARATOR},
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    time::Duration,
};
use std::collections::HashMap;
use uuid::Uuid;
use egui::viewport::{IconData};

const PADDING: f32 = 5.0;
const BTN_PADDING: f32 = 20.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const LIGHT_GRAY: Color32 = Color32::from_rgb(211, 211, 211);
//const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Todos {
    todo_list: HashMap<String, TodoEntry>,
    next_uuid: String,
    currently_edited: Option<(String, TodoEntry)>,
    selected_file: Option<PathBuf>,
    file_dialog: Option<FileDialog>,
    load_operation: bool,
    save_operation: bool,
    save_only: bool,
}

impl Default for Todos {
    fn default() -> Self {
        Self {
            todo_list: HashMap::new(),
            next_uuid: Uuid::new_v4().to_string(),
            currently_edited: None,
            selected_file: None,
            file_dialog: None,
            load_operation: false,
            save_operation: false,
            save_only: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoEntry {
    title: String,
    desc: String,
    is_done: bool,
}

impl Default for TodoEntry {
    fn default() -> Self {
        Self {
            title: "New Todo Item".to_owned(),
            desc: String::new(),
            is_done: false,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "FiraCodeNerdFont-Retina".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../fonts/FiraCodeNerdFont-Retina.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "FiraCodeNerdFont-Retina".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("FiraCodeNerdFont-Retina".to_owned());

    ctx.set_fonts(fonts);
}

#[inline]
pub fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
pub fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn heading4() -> TextStyle {
    TextStyle::Name("Heading4".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Proportional, Monospace};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(22.0, Proportional)),
        (heading2(), FontId::new(20.0, Proportional)),
        (heading3(), FontId::new(17.0, Proportional)),
        (heading4(), FontId::new(15.0, Proportional)),
        (TextStyle::Body, FontId::new(13.5, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(22.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}


impl Todos {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);

        let mut default_todo_options = Self::default();
        default_todo_options.autoload_todo_list();
        default_todo_options
    }

    pub fn render_todo_entries(&mut self, ui: &mut eframe::egui::Ui, ctx: &egui::Context) {
        let todo_list_items = self.todo_list.clone();

        for (uuid, todo_entry) in todo_list_items {
            ui.add_space(PADDING);

            let title: String = format!("▶ {}", &todo_entry.title);
            ui.colored_label(WHITE, RichText::new(title).text_style(heading2()));

            ui.add_space(PADDING);

            //let desc = Label::new(a.desc.to_string());
            //ui.add(desc);
            ui.colored_label(LIGHT_GRAY, RichText::new(&todo_entry.desc).text_style(heading3()));

            //ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.add_space(BTN_PADDING);

                let delete_entry_btn = ui.button(RichText::new("delete").text_style(heading3()));


                // idk how it does not spawn 2 buttons with clone
                // 2 means of getting a hover tooltip text on an interactive widget
                /*
                delete_entry_btn.clone().on_hover_ui(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Delete todo entry");
                    });
                });
                */
                delete_entry_btn.clone().on_hover_text("Delete todo entry");

                if delete_entry_btn.clicked() {
                    //println!("Test btn");
                    //self.todo_list.retain(|&x| x.id != )
                    self.currently_edited = Some((uuid.clone(), todo_entry.clone()));

                    if let Some((del_uuid, _del_todo_entry)) = self.currently_edited.take() {
                        self.todo_list.remove(&del_uuid);

                        //println!("uuid: {}", &self.next_uuid);
                        //println!("{:?}", self.todo_list);
                    }
                }

                ui.add_space(PADDING);

                let edit_entry_btn = ui.button(RichText::new("edit").text_style(heading3()));

                edit_entry_btn.clone().on_hover_text("Edit todo entry");

                if edit_entry_btn.clicked() {
                    //println!("Edit btn");
                    self.currently_edited = Some((uuid.clone(), todo_entry.clone()));

                }

                ui.add_space(BTN_PADDING);

                let chkbox = ui.checkbox(&mut self.todo_list.get_mut(&uuid).unwrap().is_done, RichText::new("is done?").text_style(heading3()))
                    .on_hover_text("Toggle todo status");

                if chkbox.clicked() {
                    println!("{}", todo_entry.is_done);
                    println!("{}", todo_entry.is_done);
                }
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
            ui.end_row();

            // If we're currently editing an item, we have to keep calling ctx.show_viewport_immediate
            // Remove the currently edited id and item, replace later if necessary
            if let Some((edit_uuid, mut edit_todo_entry)) = self.currently_edited.take() {
                let viewport_id = egui::ViewportId::from_hash_of(format!("edit {edit_uuid}"));
                let viewport_builder = egui::ViewportBuilder::default()
                    .with_inner_size((300.0, 300.0))
                    .with_title(format!("edit {}", edit_todo_entry.title));

                // This function is like eframe::App::update, except it can access ExampleApp as well
                let viewport_cb = |ctx: &egui::Context, _| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut edit_todo_entry.title);
                        ui.label("Description:");
                        ui.text_edit_multiline(&mut edit_todo_entry.desc);

                        if ui.button(RichText::new("Save Changes").text_style(heading3())).clicked() {
                            // Insert our changed item at the id
                            self.todo_list.insert(edit_uuid, edit_todo_entry);

                            // Set the currently edited item to nothing
                            self.currently_edited = None;
                        } 
                        else if ui.button(RichText::new("Cancel").text_style(heading3())).clicked()
                            || ctx.input(|i| i.viewport().close_requested())
                        {
                            // Set the currently edited item to nothing
                            self.currently_edited = None;
                        } 
                        else {
                            // Otherwise set the currently edited item to this item again so the window won't close
                            self.currently_edited = Some((edit_uuid, edit_todo_entry));
                        }
                    });
                };
                ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_cb);
            }
        }
    }

    pub fn add_todo_entry(&mut self) {
        // Add a default todo entry at next_uuid
        self.todo_list.insert(self.next_uuid.clone(), TodoEntry::default());

        //println!("old uuid: {}", self.next_uuid.clone());

        // And finally, generate new next_uuid
        self.next_uuid = Uuid::new_v4().to_string();

        //println!("new uuid: {}", self.next_uuid.clone());
    }

    pub fn load_todo_list(&mut self) {
        let full_file_path = self.selected_file.clone()
                                        .expect("REASON")
                                        .into_os_string()
                                        .into_string()
                                        .unwrap();

        let mut file = match File::open(&full_file_path) {
            Err(why) => panic!("Couldn't open {}: {}", full_file_path, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read {}: {}", full_file_path, why),
            Ok(_) => s.clone(),
        };

        let loaded_todo_hashmap: HashMap<String, TodoEntry> = from_str(s.as_str()).unwrap();
        //println!("loaded todo hashmap: {:#?}", loaded_todo_hashmap);
        
        self.todo_list = loaded_todo_hashmap;

        // set autoload to selected file 
        let dir_path = env::current_dir()
            .expect("REASON")
            .into_os_string()
            .into_string()
            .unwrap();

        let processed_path = dir_path + &MAIN_SEPARATOR.to_string() + "settings.conf";

        let saved_setting_hashmap = to_string(&json!({"autoload_save_file": full_file_path}));
        //println!("saving setting hashmap: {:#?}", saved_setting_hashmap);

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&processed_path)
            .expect("file did not get created");

        f.write_all(saved_setting_hashmap.expect("some random err").as_bytes()).expect("error saving todos");
    }

    pub fn save_todo_list(&mut self) {
        let full_file_path = self.selected_file.clone()
                                        .expect("REASON")
                                        .into_os_string()
                                        .into_string()
                                        .unwrap();

        let saved_todo_hashmap = to_string(&self.todo_list);
        //println!("saving todo hashmap: {:#?}", saved_todo_hashmap);

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&full_file_path)
            .expect("file did not get created");

        f.write_all(saved_todo_hashmap.expect("some random err").as_bytes()).expect("error saving todos");

        // set autoload to saved file 
        let dir_path = env::current_dir()
            .expect("REASON")
            .into_os_string()
            .into_string()
            .unwrap();

        let processed_path = dir_path + &MAIN_SEPARATOR.to_string() + "settings.conf";

        let saved_setting_hashmap = to_string(&json!({"autoload_save_file": full_file_path}));
        //println!("saving setting hashmap: {:#?}", saved_setting_hashmap);

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&processed_path)
            .expect("file did not get created");

        f.write_all(saved_setting_hashmap.expect("some random err").as_bytes()).expect("error saving todos");
    }

    // Using a config file to autoload a file if it exists for convenience
    pub fn autoload_todo_list(&mut self) {
        let dir_path = env::current_dir()
            .expect("REASON")
            .into_os_string()
            .into_string()
            .unwrap();

        let processed_path = dir_path + &MAIN_SEPARATOR.to_string() + "settings.conf";

        //println!("{:?}", &processed_path);

        if Path::new(&processed_path).exists() {
            let mut file = match File::open(&processed_path) {
                Err(why) => panic!("Couldn't open {}: {}", processed_path, why),
                Ok(file) => file,
            };

            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("Couldn't read {}: {}", processed_path, why),
                Ok(_) => s.clone(),
            };

            let loaded_setting_hashmap: HashMap<String, String> = from_str(s.as_str()).unwrap();
            //println!("loaded setting hashmap: {:#?}", loaded_setting_hashmap);

            let save_file_path = loaded_setting_hashmap["autoload_save_file"].clone();
            
            self.selected_file = Some(save_file_path.into());

            self.load_todo_list();
        }
    }
}

impl eframe::App for Todos {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_TOP, Pos2::new(-5.0, 5.0))
            .direction(Direction::TopDown);

        let toast_options = ToastOptions::default()
            .show_icon(true)
            .show_progress(true)
            .duration(Some(Duration::from_secs_f64(5.0)));

        CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button(RichText::new("File").text_style(heading2()), |ui| {
                    if ui.button(RichText::new("Load").text_style(heading4())).clicked() {
                        let filter = Box::new({
                            let ext = Some(OsStr::new("json"));
                            move |path: &Path| -> bool {path.extension() == ext}
                        });

                        // From: https://stackoverflow.com/questions/28572101/what-is-a-clean-way-to-convert-a-result-into-an-option
                        //let mut current_folder: Option<PathBuf> = std::env::current_dir().ok();

                        let mut dialog = FileDialog::open_file(self.selected_file.clone()).show_files_filter(filter);
                        //let mut dialog = FileDialog::open_file(self.selected_file.clone());
                        dialog.open();

                        self.file_dialog = Some(dialog);
                        self.load_operation = true;
                    }

                    if ui.button(RichText::new("Save").text_style(heading4())).clicked() {
                        if self.selected_file != None {
                            self.save_operation = true;
                            self.save_only = true;
                            //println!("Saving current save file");
                            //println!("file path: {:?}", self.selected_file);
                        }
                        else {
                            toasts.add(Toast {
                                kind: ToastKind::Error,
                                text: "Please load a save file\nor make a save file first via \"Save as...\"".into(),
                                options: toast_options,
                            });
                        }
                    }

                    if ui.button(RichText::new("Save as...").text_style(heading4())).clicked() {
                        let mut dialog = FileDialog::save_file(self.selected_file.clone());
                        dialog.open();

                        self.file_dialog = Some(dialog);
                        self.save_operation = true;
                    }
                });

                ui.add_space(15.0);

                ui.menu_button(RichText::new("Actions").text_style(heading2()), |ui| {
                    if ui.button(RichText::new("Add todo entry").text_style(heading4())).clicked() {
                        self.add_todo_entry();
                    }
                });

                if let Some(dialog) = &mut self.file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            //current_folder = Some(file.to_path_buf());
                            self.selected_file = Some(file.to_path_buf());
                            /*println!("{:?}", self.selected_file.clone()
                                                .expect("REASON")
                                                .into_os_string()
                                                .into_string()
                                                .unwrap());
                            */

                            if self.save_operation {
                                self.save_todo_list();
                                self.save_operation = false;
                            } 
                            else if self.load_operation {
                                self.load_todo_list();
                                self.load_operation = false;
                            }
                        }
                    }
                }

                // For saving only without filedialog
                if self.save_operation && self.save_only {
                    self.save_todo_list();
                    self.save_operation = false;
                    self.save_only = false;
                }

                ui.add_space(20.0);

                // from: https://stackoverflow.com/questions/70340483/need-to-extract-the-last-word-in-a-rust-string
                // https://stackoverflow.com/questions/54128836/how-to-access-the-file-path-separator-for-the-current-platform
                if self.selected_file != None {
                    ui.label(format!("Currently loaded: {}", self.selected_file.clone()
                                        .expect("REASON")
                                        .into_os_string()
                                        .into_string()
                                        .unwrap()
                                        .split(MAIN_SEPARATOR)
                                        .last()
                                        .unwrap()));
                }
            });

            ui.add(Separator::default());

            egui::containers::scroll_area::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                self.render_todo_entries(ui, ctx);
            });

            // show toasts notifications
            toasts.show(ctx);
        });
    }
}
