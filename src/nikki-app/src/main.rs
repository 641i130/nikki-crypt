use eframe::{
    egui::{
        self, menu, Button, CentralPanel, Layout, ScrollArea, SidePanel, TextEdit, TopBottomPanel,
    },
    epi::{self, App},
    run_native, NativeOptions,
};
use std::{
    fs::{self, read_dir, remove_file, File},
    io::Write,
    path::Path,
};

// Variables in app
struct NikkiApp {
    active_window: u8,
    key: String,
    unlocked: bool,
    title: String,
    body: String,
}

// Set default values for app start
impl NikkiApp {
    fn new() -> NikkiApp {
        NikkiApp {
            active_window: 0,
            key: "".to_string(),
            unlocked: false,
            title: "".to_string(),
            body: "".to_string(),
        }
    }
}

impl App for NikkiApp {
    // Names window
    fn name(&self) -> &str {
        "nikki-crypt"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // Only renders top panel when the app is unlocked
        if self.unlocked {
            TopBottomPanel::top("top_panel").show(ctx, |ui| {
                menu::bar(ui, |ui| {
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        if ui.button("File").clicked() {
                            self.active_window = 1;
                        }
                        if ui.button("Settings").clicked() {
                            self.active_window = 2;
                        }
                        if ui.button("Lock").clicked() {
                            self.unlocked = false;
                            self.active_window = 0;
                        }
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                });
            });
        }

        match self.active_window {
            // Login window
            0 => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Log in pls");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.add(
                            TextEdit::singleline(&mut self.key)
                                .desired_width(100.00)
                                .password(true),
                        );
                        if ui.button("Log in").clicked() {
                            // TODO: PLEASE MAKE SOMETHING SAFER THAN THIS 
                            if self.key == "1234" {
                                self.unlocked = true;
                                self.active_window = 1;
                                self.key = "".to_string();
                            } else {
                                self.key = "".to_string();
                            }
                        }
                    });
                });
            }
            // File window
            1 => {
                // Sidebar with all of the available files
                SidePanel::left("journal_entries")
                    .max_width(150.00)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.separator();
                                if ui.add(Button::new("New Entry").frame(false)).clicked() {
                                    self.title = "".to_string();
                                    self.body = "".to_string();
                                }
                                ui.separator();
                                // Get paths of all files in entries folder
                                let paths = read_dir("./entries").unwrap();
                                // Loops through result of read_dir and creates buttons to access each file
                                for path in paths {
                                    // Converts path of file to string
                                    let path =
                                        String::from(path.unwrap().path().display().to_string());
                                    // Finds indexes so string can be sliced for the file name
                                    let start = path.find("/entries").unwrap_or(0) + 9;
                                    let end = path.find(".txt").unwrap_or(path.len());
                                    let button_title_name = String::from(&path[start..end]);
                                    // Button to access file contents
                                    if ui
                                        .add(Button::new(&button_title_name).frame(false))
                                        .clicked()
                                    {
                                        self.title = button_title_name;
                                        let txt_contents = fs::read_to_string(path)
                                            .expect("Unable to get contents of file");
                                        self.body = txt_contents;
                                    }
                                    ui.separator();
                                }
                            });
                    });
                // Panel for file content;
                CentralPanel::default().show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Title");
                        ui.horizontal(|ui| {
                            ui.add(TextEdit::singleline(&mut self.title).desired_width(300.00));
                        });
                        ui.separator();
                        ui.label("Body");
                        ui.horizontal(|ui| {
                            ui.add(
                                TextEdit::multiline(&mut self.body)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(24),
                            );
                        });
                        ui.separator();
                        // Bottom bar for the save and delete buttons
                        menu::bar(ui, |ui| {
                            ui.with_layout(Layout::right_to_left(), |ui| {
                                if ui.button("Delete").clicked() {
                                    // Fixes the issue where the program would crash if file doesn't exist
                                    if Path::new(&format!("./entries/{}.txt", self.title)).exists()
                                    {
                                        remove_file(format!("./entries/{}.txt", self.title))
                                            .expect("Error deleting file");
                                    }
                                }
                                if ui.button("Save").clicked() {
                                    // If title empty then generate a title named New Entry {} where {} = available integer
                                    if self.title == "" {
                                        let mut counter: u32 = 1;
                                        // Infinite loop until number is available
                                        loop {
                                            if !Path::new(&format!(
                                                "./entries/New Entry {}.txt",
                                                counter
                                            ))
                                            .exists()
                                            {
                                                self.title = format!("New Entry {}", counter);
                                                break;
                                            }
                                            counter += 1;
                                        }
                                    }
                                    // Create new file or overwrite current file
                                    let mut new_file =
                                        File::create(format!("./entries/{}.txt", self.title))
                                            .expect("Unable to create file");
                                    new_file
                                        .write_all(self.body.as_bytes())
                                        .expect("Unable to write to file");
                                };
                            });
                        });
                    });
                });
            }
            // Settings window
            2 => {
                CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Settings");
                    ui.separator();
                });
            }
            _ => {
                panic!("Panik");
            }
        }
    }
}

fn main() {
    let app = NikkiApp::new();
    let mut win_option = NativeOptions::default();
    // Sets window size
    win_option.resizable = false;
    win_option.initial_window_size = Some(egui::Vec2 {
        x: 640.00,
        y: 480.00,
    });
    run_native(Box::new(app), win_option);
}
