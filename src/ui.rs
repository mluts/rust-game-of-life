use cairo::Context;
// use gio::prelude::*;
use gtk::prelude::*;

use crate::game::Game;
use crate::grid;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
struct GridProperties {
    size: (f64, f64),
    // lwidth: f64,
    grid_size: (u32, u32),
    grid_rgb: (f64, f64, f64),
    squares: Vec<grid::Square>,
}

pub trait Render<E> {
    fn render(&self, ctx: &Context) -> Result<(), E>;
}

fn draw(props: &Rc<RefCell<GridProperties>>, ctx: &Context) -> Inhibit {
    let props = props.borrow();
    let lwidth = 1.0 / props.size.0;

    ctx.scale(props.size.0, props.size.1);

    ctx.set_line_width(lwidth);
    ctx.set_source_rgb(props.grid_rgb.0, props.grid_rgb.1, props.grid_rgb.2);

    for line in grid::grid(props.grid_size.0, props.grid_size.1) {
        line.render(ctx).expect("Line render failed!");
    }
    ctx.stroke();

    for sq in props.squares.iter() {
        sq.render(ctx).unwrap()
    }
    ctx.fill();

    Inhibit(false)
}

// fn move_square(props: &Rc<RefCell<GridProperties>>) {
//     let mut props = props.borrow_mut();

//     if props.squares.len() == 0 {
//         let square = grid::Square::new(0, props.grid_size.0, 0, props.grid_size.1);
//         props.squares.push(square);
//     } else {
//         let mut square = props.squares.remove(0);

//         square.col.0 = (square.col.0 + 1) % square.col.1;
//         square.row.0 = (square.row.0 + 1) % square.row.1;
//         props.squares.push(square);
//     }
// }

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .resizable(false)
        .application(app)
        .build();

    let game = Rc::new(RefCell::new(Game::new(50)));

    let props = Rc::new(RefCell::new(GridProperties {
        size: (700.0, 700.0),
        grid_size: (100, 100),
        grid_rgb: (0.9, 0.9, 0.9),
        squares: game.borrow().squares(),
    }));

    let drawing_area = Box::new(gtk::DrawingArea::new)();
    window.add(&drawing_area);

    let dprops = Rc::clone(&props);
    drawing_area.connect_draw(move |_da, ctx| draw(&dprops, ctx));

    window.set_default_size(700, 700);
    window.show_all();

    gtk::timeout_add(300, move || {
        let mut g = game.borrow_mut();
        g.forecast();
        g.move_to_future();
        props.borrow_mut().squares = g.squares();
        drawing_area.queue_draw();
        Continue(true)
    });
}
