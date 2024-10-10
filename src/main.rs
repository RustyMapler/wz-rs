extern crate wz;

use std::io;
use wz::MainWindow;

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
