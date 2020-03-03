// use cairo::Context;
use gio::prelude::*;
use std::env::args;

pub mod game;
pub mod grid;
mod ui;

fn build_app() -> gtk::Application {
    gtk::Application::new(Some("com.github.mluts.game-of-life"), Default::default())
        .map_err(|e| format!("Initialization failed: {}", e))
        .unwrap()
}

fn main() {
    let app = build_app();

    app.connect_activate(ui::build_ui);

    app.run(&args().collect::<Vec<_>>());
}
