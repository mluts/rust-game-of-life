use cairo::Context;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use crate::game::Game;
// use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

struct UIProperties {
    grid_rgb: (f64, f64, f64),
    game: Game,
}

pub trait Render<E> {
    fn render(&self, ctx: &Context) -> Result<(), E>;
}

fn draw(props: &Arc<RwLock<UIProperties>>, da: &gtk::DrawingArea, ctx: &Context) -> Inhibit {
    let props = props.read().unwrap();
    let size = da.get_allocation();
    let lwidth = 1.0 / size.width as f64;

    ctx.scale(size.width as f64, size.height as f64);

    ctx.set_line_width(lwidth);
    ctx.set_source_rgb(props.grid_rgb.0, props.grid_rgb.1, props.grid_rgb.2);

    for sq in props.game.squares().iter() {
        sq.render(ctx).unwrap()
    }
    ctx.fill();

    Inhibit(false)
}

fn build_app_menu() -> gio::Menu {
    let menu = gio::Menu::new();

    menu.append(Some("Reset"), Some("app.reset"));
    menu.append(Some("Quit"), Some("app.quit"));

    menu
}

fn add_menu(
    app: &gtk::Application,
    window: &gtk::ApplicationWindow,
    props: &Arc<RwLock<UIProperties>>,
) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(@weak window => move |_, _| {
        window.destroy();
    }));

    let reset = gio::SimpleAction::new("reset", None);
    let props = Arc::clone(props);
    reset.connect_activate(move |_, _| {
        let mut props = props.write().unwrap();
        props.game.generate_cells();
    });

    app.set_app_menu(Some(&build_app_menu()));
    app.add_action(&quit);
    app.add_action(&reset);
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .resizable(true)
        .application(app)
        .build();

    let props = Arc::new(RwLock::new(UIProperties {
        game: Game::new(200, 200, 200 * 200 / 8),
        grid_rgb: (0.9, 0.9, 0.9),
    }));

    add_menu(app, &window, &props);

    let drawing_area = Box::new(gtk::DrawingArea::new)();
    window.add(&drawing_area);

    let dprops = Arc::clone(&props);
    drawing_area.connect_draw(move |da, ctx| draw(&dprops, da, ctx));

    window.set_default_size(500, 500);
    window.show_all();

    let props = Arc::clone(&props);

    gtk::timeout_add(300, move || {
        let mut props = props.write().unwrap();
        props.game.move_cells();
        drawing_area.queue_draw();
        Continue(true)
    });
}
