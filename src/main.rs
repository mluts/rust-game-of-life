// use cairo::Context;
use gio::prelude::*;
// use gtk::prelude::*;
use std::env::args;

pub mod grid;
mod ui;

fn main() {
    let app = gtk::Application::new(Some("com.github.mluts.game-of-life"), Default::default())
        .map_err(|e| format!("Initialization failed: {}", e))
        .unwrap();

    app.connect_activate(ui::build_ui);

    app.run(&args().collect::<Vec<_>>());
}
