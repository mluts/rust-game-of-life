use cairo::Context;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use crate::game::Game;
use crate::grid;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
struct GridProperties {
    grid_size: (u32, u32),
    grid_rgb: (f64, f64, f64),
    squares: Vec<grid::Square>,
}

pub trait Render<E> {
    fn render(&self, ctx: &Context) -> Result<(), E>;
}

fn draw(props: &Rc<RefCell<GridProperties>>, da: &gtk::DrawingArea, ctx: &Context) -> Inhibit {
    let props = props.borrow();
    let size = da.get_allocation();
    let lwidth = 1.0 / size.width as f64;

    ctx.scale(size.width as f64, size.height as f64);

    ctx.set_line_width(lwidth);
    ctx.set_source_rgb(props.grid_rgb.0, props.grid_rgb.1, props.grid_rgb.2);

    for sq in props.squares.iter() {
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

fn add_menu(app: &gtk::Application, window: &gtk::ApplicationWindow, game: &Rc<RefCell<Game>>) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(@weak window => move |_, _| {
        window.destroy();
    }));

    let reset = gio::SimpleAction::new("reset", None);
    let reset_game = Rc::clone(game);
    reset.connect_activate(move |_, _| {
        reset_game.borrow_mut().reset();
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

    let game = Rc::new(RefCell::new(Game::new(200)));

    let props = Rc::new(RefCell::new(GridProperties {
        grid_size: (200, 200),
        grid_rgb: (0.9, 0.9, 0.9),
        squares: game.borrow().squares(),
    }));

    add_menu(app, &window, &game);

    let drawing_area = Box::new(gtk::DrawingArea::new)();
    window.add(&drawing_area);

    let dprops = Rc::clone(&props);
    drawing_area.connect_draw(move |da, ctx| draw(&dprops, da, ctx));

    window.set_default_size(500, 500);
    window.show_all();

    gtk::timeout_add(500, move || {
        let mut g = game.borrow_mut();
        g.forecast();
        g.move_to_future();
        props.borrow_mut().squares = g.squares();
        drawing_area.queue_draw();
        Continue(true)
    });
}
