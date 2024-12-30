#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::Color32;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0])
            .with_resizable(false),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "file-image-converter implementation",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(1.5); // Set zoom to 200%
            Ok(Box::<FileImageConverterApp>::default())
        }),
    )
}

struct FileImageConverterApp {
    output_file_path: String,
    input_path: Option<String>,
    show_error: Option<String>
}

impl Default for FileImageConverterApp {
    fn default() -> Self {
        Self {
            output_file_path: String::new(),
            input_path: None,
            show_error: None
        }
    }
}

impl eframe::App for FileImageConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_error.is_some() {
                error_pop_up(ui, &self.show_error.clone().unwrap(), &mut self.show_error);
                return;
            }
            ui.heading("file-image-converter implementation");
            ui.horizontal(|ui| {
                ui.label("Drag-and-drop files onto the window!");
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.input_path = Some(path.display().to_string());
                    }
                }

            });
            ui.separator();
            if self.input_path.is_some() {
                ui.group(|ui| {
                    ui.label("Dropped files:");
                    ui.label(self.input_path.clone().unwrap());
                });
            }
            ui.separator();
            if ui.button("Convert").clicked() && self.input_path.is_some() {
                match rfd::FileDialog::new().save_file() {
                    Some(path) => {
                        self.output_file_path = path.display().to_string();
                    }
                    None => {
                        self.show_error = Some("Failed to save file.".parse().unwrap());
                        self.input_path = None;
                        return;
                    }
                }
                if self.output_file_path.ends_with(".png") && self.input_path.clone().unwrap().ends_with(".png") {
                    self.show_error = Some("You can't convert a PNG to a PNG. You can only convert from PNG to other formats.".parse().unwrap());
                    self.input_path = None;
                    return;
                }
                if self.output_file_path.ends_with(".png") && !self.input_path.clone().unwrap().ends_with(".png") {
                    file_image_converter::convert_file_to_png(
                        &*self.input_path.clone().unwrap(),
                        &*self.output_file_path.clone()
                    ).expect("Failed to convert file to PNG");

                    self.input_path = None;
                } else {
                    if self.input_path.clone().unwrap().ends_with(".png") {
                        file_image_converter::convert_png_to_file(
                            &*self.input_path.clone().unwrap(),
                            Option::from(&*self.output_file_path.clone())
                        ).expect("Failed to convert PNG to file");
                        self.input_path = None;
                    } else {
                        self.show_error = Some("The file you picked is not a PNG. you can only convert PNG files to other formats.".parse().unwrap());
                        self.input_path = None;
                    }
                }
            }
        });
    }
}

fn error_pop_up(ui: &mut egui::Ui, error_message: &str, show_error: &mut Option<String>) {
    egui::Window::new("Error")
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            ui.colored_label(Color32::RED, error_message);
            if ui.button("Okay, I understand.").clicked() {
                *show_error = None;
            }
        });
}