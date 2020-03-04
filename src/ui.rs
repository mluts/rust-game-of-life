use cairo::Context;

use gio::prelude::*;
use gtk::prelude::*;

use glib::clone;

use crate::game::Game;
use std::collections::{HashSet, VecDeque};
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, RwLock};

struct UIProperties {
    grid_rgb: (f64, f64, f64),
    game: Game,
    timeout_id: Option<glib::source::SourceId>,
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

fn run_game_loop() -> (Arc<RwLock<UIProperties>>, SyncSender<u32>) {
    let (resize_tx, resize_rx) = sync_channel::<u32>(0);

    let props = Arc::new(RwLock::new(UIProperties {
        game: Game::new(200, 200, 200 * 200 / 8),
        grid_rgb: (0.9, 0.9, 0.9),
        timeout_id: None,
    }));

    let mut game_copy: Game;

    let pprops = Arc::clone(&props);
    {
        game_copy = pprops.read().unwrap().game.clone();
    }
    let mut next_cells: VecDeque<HashSet<u32>> = VecDeque::new();
    next_cells.push_back(game_copy.cells());

    std::thread::spawn(move || loop {
        match resize_rx.try_recv() {
            Ok(n) => {
                let mut p = pprops.write().unwrap();
                p.game = Game::new(n, n, n * n / 8);
                game_copy = p.game.clone();
                next_cells.clear();
                next_cells.push_back(game_copy.cells());
            }
            _ => (),
        }
        {
            let cells = next_cells.pop_front().unwrap();
            let mut props = pprops.write().unwrap();
            props.game.set_cells(cells);
        }

        game_copy.move_cells();
        next_cells.push_back(game_copy.cells());
    });

    (props, resize_tx)
}

fn run_render_loop(props: &Arc<RwLock<UIProperties>>, drawing_area: &gtk::DrawingArea, delay: u32) {
    let props = Arc::clone(props);

    {
        let mut p = props.write().unwrap();
        match std::mem::replace(&mut p.timeout_id, None) {
            Some(tid) => {
                glib::source::source_remove(tid);
            }
            None => (),
        }
    }

    let drawing_area = drawing_area.clone();

    let source_id = gtk::timeout_add(delay, move || {
        drawing_area.queue_draw();
        Continue(true)
    });

    {
        let mut p = props.write().unwrap();
        p.timeout_id = Some(source_id);
    }
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .resizable(true)
        .application(app)
        .build();

    let (props, resize_tx) = run_game_loop();

    let drawing_area = Box::new(gtk::DrawingArea::new)();
    add_menu(app, &window, &props, &drawing_area, move |n| {
        resize_tx.send(n).unwrap();
    });

    let dprops = Arc::clone(&props);
    drawing_area.connect_draw(move |da, ctx| draw(&dprops, da, ctx));

    window.set_default_size(500, 500);
    window.show_all();

    run_render_loop(&props, &drawing_area, 100);
}
