use crate::{ArcDynamicWzNode, WzFile, WzVersion};
use eframe::egui::{self, Direction, Layout, ScrollArea};
use rfd::FileDialog;
use std::io::Error;

pub struct MainWindow {
    pub window_name: String,
    pub wz_file: Option<WzFile>,
    pub wz_node: Option<ArcDynamicWzNode>,
    pub selected_wz_node: Option<ArcDynamicWzNode>,
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
            self.ui_menu_bar(ui);
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

    fn ui_menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Open File").clicked() {
                let _ = self.open_file();
            }
        });
    }

    fn ui_main_content(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.group(|ui| {
                self.ui_wz_directory(ui);
            });

            ui.group(|ui| {
                self.ui_wz_focused(ui);
            });
        });
    }

    fn ui_wz_directory(&mut self, ui: &mut egui::Ui) {
        let wz_node = self.wz_node.clone();

        ScrollArea::both()
            .auto_shrink(false)
            .max_width(ui.available_width() * 0.5)
            .show(ui, |ui| {
                if let Some(wz_node) = wz_node {
                    self.ui_wz_node_recursive(ui, &wz_node);
                }
            });
    }

    fn ui_wz_focused(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if let Some(selected_wz_node) = &self.selected_wz_node {
                    ui.label(format!("Selected node: {}", selected_wz_node));
                } else {
                    ui.label("No node selected.");
                }
            },
        );
    }

    fn ui_wz_node_recursive(&mut self, ui: &mut egui::Ui, node: &ArcDynamicWzNode) {
        ui.collapsing(node.name.clone(), |ui| {
            if node.children.is_empty() {
                if ui.label(format!("{}", node.value)).clicked() {
                    self.selected_wz_node = Some(node.clone());
                }
            }

            for child in node.children.values() {
                self.ui_wz_node_recursive(ui, child);
            }
        });
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
