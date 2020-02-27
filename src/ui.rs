use cairo::Context;
// use gio::prelude::*;
use gtk::prelude::*;

use crate::grid;

pub trait Render<E> {
    fn render(&self, ctx: &Context) -> Result<(), E>;
}

fn draw(_da: &gtk::DrawingArea, ctx: &Context) -> Inhibit {
    let size = (500f64, 500f64);
    let lwidth = 1.0 / size.0;
    let gr_size = (50, 50);

    ctx.scale(size.0, size.1);

    ctx.set_line_width(lwidth);
    ctx.set_source_rgb(0.9, 0.9, 0.9);

    for line in grid::grid(gr_size.0, gr_size.1) {
        line.render(ctx).expect("Line render failed!");
    }
    ctx.stroke();

    grid::Square::new(5, gr_size.0, 5, gr_size.1)
        .render(ctx)
        .expect("square render failed!");

    ctx.fill();

    // ctx.stroke();
    // ctx.set_line_width(0.01);
    // ctx.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    // ctx.rectangle(10.0, 10.0, 10.0, 10.0);
    // ctx.fill();

    // ctx.set_line_width(0.01);
    // ctx.set_source_rgb(0.9, 0.9, 0.9);
    // ctx.rectangle(0.01, 0.01, 0.5, 0.5);

    Inhibit(false)
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .resizable(false)
        .application(app)
        .build();
    let drawing_area = Box::new(gtk::DrawingArea::new)();

    drawing_area.connect_draw(draw);
    window.set_default_size(500, 500);
    window.add(&drawing_area);
    window.show_all();
}
