use crate::grid::Square;
use rand::{prelude::*, Rng};
use std::collections::HashSet;

pub struct Cell {
    pub curr_state: bool,
    pub future_state: Option<bool>,
}

pub struct Game {
    pub cells: Vec<Cell>,
    size: u32,
    cells_cnt: usize,
}

fn generate_cells(cells_n: u32, n: u32) -> Vec<Cell> {
    let mut rng = thread_rng();
    let alive_cells: HashSet<u32> = (0..n).map(|_| rng.gen_range(0, cells_n)).collect();

    (0..cells_n)
        .map(|i| {
            let mut alive = false;
            if alive_cells.contains(&i) {
                alive = true
            }
            let mut cell = Cell::default();
            cell.curr_state = alive;
            cell
        })
        .collect::<Vec<Cell>>()
}

impl Cell {
    pub fn default() -> Cell {
        Cell {
            curr_state: false,
            future_state: None,
        }
    }
}

impl Game {
    pub fn new(size: u32) -> Game {
        let cells_cnt = (size * size) as usize;
        let cells = generate_cells(cells_cnt as u32, 200);

        Game {
            cells: cells,
            size: size,
            cells_cnt: cells_cnt,
        }
    }

    pub fn adjacent_cells(&self, i: i32) -> Vec<&Cell> {
        let size = self.size as i32;
        vec![
            i - 1,
            i + 1,
            i - size,
            i + size,
            i - size - 1,
            i - size + 1,
            i + size - 1,
            i + size + 1,
        ]
        .into_iter()
        .filter(|n| n >= &0 && n < &(self.cells_cnt as i32))
        .map(|n| self.cells.get(n as usize).unwrap())
        .collect::<Vec<&Cell>>()
    }

    pub fn cell_x(&self, i: u32) -> u32 {
        i % self.size
    }

    pub fn cell_y(&self, i: u32) -> u32 {
        i / self.size
    }

    pub fn forecast(&mut self) {
        for i in 0..=(self.cells_cnt - 1) {
            let alive_neighbors = self
                .adjacent_cells(i as i32)
                .iter()
                .filter(|c| c.curr_state)
                .count();

            let cell = self.cells.get_mut(i).unwrap();

            match alive_neighbors {
                0..=1 => match cell.curr_state {
                    true => cell.future_state = Some(false),
                    false => cell.future_state = Some(false), // false => cell.future_state = Some(false),
                },
                2 => match cell.curr_state {
                    true => cell.future_state = Some(true),
                    false => cell.future_state = Some(false),
                },
                3 => match cell.curr_state {
                    true => cell.future_state = Some(true),
                    false => cell.future_state = Some(true),
                },
                4..=8 => match cell.curr_state {
                    true => cell.future_state = Some(false),
                    false => cell.future_state = Some(false),
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn move_to_future(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.curr_state = cell.future_state.unwrap();
            cell.future_state = None;
        }
    }

    pub fn squares(&self) -> Vec<Square> {
        (0..self.cells_cnt)
            .filter(|i| self.cells.get(*i as usize).unwrap().curr_state)
            .map(|i| {
                Square::new(
                    self.cell_x(i as u32),
                    self.size,
                    self.cell_y(i as u32),
                    self.size,
                )
            })
            .collect::<Vec<Square>>()
    }
}
