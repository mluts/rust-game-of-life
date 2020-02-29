use crate::ui;
use cairo::Context;
use serde::{Deserialize, Serialize};

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Square {
    pub col: (u32, u32),
    pub row: (u32, u32),
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let a = Point { x: x1, y: y1 };
        let b = Point { x: x2, y: y2 };

        Self { a, b }
    }
}

impl Square {
    pub fn new(col: u32, cols: u32, row: u32, rows: u32) -> Square {
        Square {
            col: (col, cols),
            row: (row, rows),
        }
    }
}

fn sequence(start: f64, end: f64, step: f64) -> Vec<f64> {
    let mut x: f64 = start;
    let mut els: Vec<f64> = vec![];

    while x <= end {
        els.push(x);
        x += step;
    }

    els
}

fn vlines_seq(cols: u32) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    let step = 1.0 / cols as f64;

    for x in sequence(step, 1.0, step) {
        lines.push(Line::new(x, 0.0, x, 1.0))
    }

    lines
}

fn hlines_seq(rows: u32) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    let step = 1.0 / rows as f64;

    for y in sequence(step, 1.0, step) {
        lines.push(Line::new(0.0, y, 1.0, y))
    }

    lines
}

pub fn grid(cols: u32, rows: u32) -> Vec<Line> {
    let mut vlines = vlines_seq(cols);
    vlines.extend(hlines_seq(rows));

    vlines
}

impl ui::Render<String> for Line {
    fn render(&self, ctx: &Context) -> Result<(), String> {
        ctx.move_to(self.a.x, self.a.y);
        ctx.line_to(self.b.x, self.b.y);

        Ok(())
    }
}

impl ui::Render<String> for Square {
    fn render(&self, ctx: &Context) -> Result<(), String> {
        let xwidth = 1.0 / self.col.1 as f64;
        let ywidth = 1.0 / self.row.1 as f64;
        let x = 0.0 + xwidth * self.col.0 as f64;
        let y = 0.0 + ywidth * self.row.0 as f64;

        // println!("Square::render {:?}", &self);
        ctx.rectangle(x, y, xwidth, ywidth);
        Ok(())
    }
}
