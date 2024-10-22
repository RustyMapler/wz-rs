use eframe::egui::{self, Color32, Direction, Layout, RichText, ScrollArea};
use rfd::FileDialog;
use std::io::{self, Error};
use wz::{parse_canvas, ArcWzNode, WzCanvas, WzFile, WzImage, WzValueCast, WzVersion};

pub struct MainWindow {
    pub window_name: String,
    pub wz_file: Option<WzFile>,
    pub wz_node: Option<ArcWzNode>,
    pub selected_wz_node: Option<ArcWzNode>,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            window_name: "Wz Explorer".to_owned(),
            wz_file: None,
            wz_node: None,
            selected_wz_node: None,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_main_menu_bar(ui);
            self.ui_main_content(ui);
        });
    }
}

impl MainWindow {
    pub fn run(&self) -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
            ..Default::default()
        };

        eframe::run_native(
            &self.window_name,
            options,
            Box::new(|cc| {
                // This gives us image support:
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::<MainWindow>::default())
            }),
        )
    }

    fn ui_main_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Open File").clicked() {
                let _ = self.open_file();
            }
        });
    }

    fn ui_main_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.group(|ui| {
                self.ui_wz_node_directory(ui);
            });

            ui.group(|ui| {
                self.ui_wz_node_selection(ui);
            });
        });
    }

    fn ui_wz_node_directory(&mut self, ui: &mut egui::Ui) {
        ScrollArea::both()
            .auto_shrink(false)
            .max_width(ui.available_width() * 0.5)
            .show(ui, |ui| {
                if let Some(wz_node) = self.wz_node.clone() {
                    self.ui_wz_node_directory_recursive(ui, &wz_node);
                }
            });
    }

    fn ui_wz_node_directory_recursive(&mut self, ui: &mut egui::Ui, node: &ArcWzNode) {
        let heading = node_heading(node);
        let collapsing_section = ui.collapsing(heading, |ui| {
            for child in node.children.values() {
                self.ui_wz_node_directory_recursive(ui, child);
            }

            if node.children.is_empty() {
                ui.label(RichText::new(format!("{}", node.value)).color(Color32::LIGHT_GRAY));
            }
        });

        if collapsing_section.header_response.clicked() {
            self.selected_wz_node = Some(node.clone());
        }
    }

    fn ui_wz_node_selection(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if let Some(selected_wz_node) = &self.selected_wz_node {
                    let value = selected_wz_node.value.clone();

                    if let Some(canvas) = value.as_canvas() {
                        let reader = self.wz_file.as_ref().unwrap().reader.clone();
                        let image = parse_canvas(canvas, reader).unwrap();

                        let name = selected_wz_node.name.clone();
                        let size = [image.width as usize, image.height as usize];
                        let data = image.data.clone();

                        let texture = ui.ctx().load_texture(
                            name,
                            egui::ColorImage::from_rgba_unmultiplied(size, &data),
                            egui::TextureOptions::default(),
                        );

                        ui.image(&texture);
                    } else {
                        ui.label(format!("Selected node: {}", selected_wz_node));
                    }
                } else {
                    ui.label("No node selected.");
                }
            },
        );
    }

    fn open_file(&mut self) -> Result<(), Error> {
        if let Some(path) = FileDialog::new().pick_file() {
            // Create a new wz file
            let mut wz_file = WzFile::new(path.display().to_string().as_str(), WzVersion::GMS);

            // Open it
            wz_file.open()?;

            // Make sure to parse the root directory
            let node = wz_file.parse_root_directory()?;

            // Set the file and node
            self.wz_file = Some(wz_file);
            self.wz_node = Some(node);
        }

        Ok(())
    }
}

pub fn node_heading(node: &ArcWzNode) -> String {
    format!("{}({})", node.name, node.offset)
}

fn main() -> io::Result<()> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_module_level("wz", log::LevelFilter::Info)
        .with_module_level("eframe", log::LevelFilter::Error)
        .init()
        .unwrap();

    let app = MainWindow::default();

    let _result = app.run();

    Ok(())
}
