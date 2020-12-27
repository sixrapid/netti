use std::env::args;

use gio::{ApplicationExt, prelude::ApplicationExtManual};

mod gui;
mod netctl;

fn main() {
    // Launch the GUI
    let application = gtk::Application::new(
        Some("com.github.sixrapid.netti"),
        Default::default(),
    ).expect("Initialization failed...");

    application.connect_startup(|app| {
        gui::build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}