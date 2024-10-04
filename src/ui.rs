use core::f32;
use std::io::Error;

use eframe::egui::{self, ScrollArea};
use rfd::FileDialog;

use crate::{wz_node, ArcDynamicWzNode, WzFile, WzVersion};

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

impl MainWindow {
    fn open_file(&mut self) -> Result<(), Error> {
        if let Some(path) = FileDialog::new().pick_file() {
            let mut wz_file = WzFile::new(path.display().to_string().as_str(), WzVersion::GMS);
            wz_file.open()?;

            let node = wz_file.parse_root_directory()?;

            self.wz_file = Some(wz_file);
            self.wz_node = Some(node);
        }

        Ok(())
    }

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

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            self.menu_file(ui);
        });
    }

    fn menu_file(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open File").clicked() {
            let _ = self.open_file();
        }
    }

    fn main_content(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .auto_shrink(false)
            .show(ui, |ui| {
                if let Some(wz_node) = &self.wz_node {
                    show_node_ui(ui, wz_node);
                }
            });
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.menu_bar(ui);
            self.main_content(ui);
        });
    }
}

fn show_node_ui(ui: &mut egui::Ui, node: &ArcDynamicWzNode) {
    ui.collapsing(node.name.clone(), |ui| {
        if ui.label(format!("Value: {}", node.value)).clicked() {}

        for child in node.children.values() {
            show_node_ui(ui, child);
        }
    });
}
