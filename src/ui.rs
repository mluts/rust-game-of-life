use cairo::Context;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use crate::game::Game;
use std::sync::mpsc::{sync_channel, Receiver};
use std::sync::{Arc, RwLock};

struct UIProperties {
    grid_rgb: (f64, f64, f64),
    game: Game,
}

pub trait Render<E> {
    fn render(&self, ctx: &Context) -> Result<(), E>;
}

fn draw(
    props: &Arc<RwLock<UIProperties>>,
    render_rx: &Receiver<()>,
    da: &gtk::DrawingArea,
    ctx: &Context,
) -> Inhibit {
    render_rx.recv().unwrap();

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

fn build_size_entry<F: Fn(u32) + 'static>(f: F) -> gtk::Entry {
    let entry = gtk::Entry::new();

    entry.connect_activate(move |e| match e.get_buffer().get_text().parse::<u32>() {
        Ok(n) => f(n),
        _ => (),
    });

    entry
}

fn add_menu<F: Fn(u32) + 'static>(
    app: &gtk::Application,
    window: &gtk::ApplicationWindow,
    props: &Arc<RwLock<UIProperties>>,
    drawing_area: &gtk::DrawingArea,
    resize_f: F,
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

    let window_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let control_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    control_box.pack_start(&gtk::Label::new(Some("Size")), false, false, 0);
    control_box.pack_start(&build_size_entry(resize_f), false, false, 0);
    window_box.pack_start(&control_box, false, false, 0);
    window_box.pack_start(drawing_area, true, true, 0);

    window.add(&window_box);
    app.set_app_menu(Some(&build_app_menu()));
    app.add_action(&quit);
    app.add_action(&reset);
}

fn run_game_loop() -> (Arc<RwLock<UIProperties>>, Receiver<()>) {
    let (render_tx, render_rx) = sync_channel(0);

    let props = Arc::new(RwLock::new(UIProperties {
        game: Game::new(200, 200, 200 * 200 / 8),
        grid_rgb: (0.9, 0.9, 0.9),
    }));

    let pprops = Arc::clone(&props);

    std::thread::spawn(move || loop {
        {
            let mut props = pprops.write().unwrap();
            props.game.move_cells();
        }
        render_tx.send(()).unwrap();
    });

    (props, render_rx)
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .resizable(true)
        .application(app)
        .build();

    let (props, render_rx) = run_game_loop();

    let drawing_area = Box::new(gtk::DrawingArea::new)();
    let pprops = Arc::clone(&props);
    add_menu(app, &window, &props, &drawing_area, move |n| {
        let mut p = pprops.write().unwrap();
        p.game = Game::new(n, n, n * n / 8);
    });

    let dprops = Arc::clone(&props);
    drawing_area.connect_draw(move |da, ctx| draw(&dprops, &render_rx, da, ctx));

    window.set_default_size(500, 500);
    window.show_all();

    gtk::timeout_add(100, move || {
        drawing_area.queue_draw();
        Continue(true)
    });
}
