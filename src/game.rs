use crate::grid;
use rand::{prelude::*, Rng};
use std::collections::HashSet;

pub struct Game {
    grid: HashSet<u32>,
    width: u32,
    height: u32,
    initial_cells: u32,
}

impl Game {
    pub fn new(width: u32, height: u32, initial_cells: u32) -> Game {
        let mut game = Game {
            width,
            height,
            initial_cells,
            grid: HashSet::new()
        };
        game.generate_cells();
        game
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn cells_cnt(&self) -> u32 {
        self.width * self.height
    }

    pub fn generate_cells(&mut self) {
        let mut rng = thread_rng();
        let alive_cells: HashSet<u32> = (0..)
            .map(|_| rng.gen_range(0, self.cells_cnt()))
            .take(self.initial_cells as usize)
            .collect();

        self.grid = alive_cells;
    }

    fn adjacent_cells(&self, i: u32) -> Vec<u32> {
        let size = self.width as i32;
        let i = i as i32;
        vec![
            i - 1,
            i - size,
            i - size - 1,
            i - size + 1,
            i + 1,
            i + size,
            i + size - 1,
            i + size + 1,
        ]
        .into_iter()
        .map(|i| i as u32)
        .filter(|n| n >= &0 && n < &(self.cells_cnt()) && self.grid.contains(n))
        .collect::<Vec<u32>>()
    }

    pub fn forecast_cell(&self, i: u32) -> bool {
        let neighbours = self.adjacent_cells(i);
        let cell_alive = self.grid.contains(&i);

        match neighbours.len() {
            0..=1 => match cell_alive {
                true => false,
                false => false,
            },
            2 => match cell_alive {
                true => true,
                false => false,
            },
            3 => match cell_alive {
                true => true,
                false => true,
            },
            4..=8 => match cell_alive {
                true => false,
                false => false,
            },
            _ => unreachable!(),
        }
    }

    pub fn set_cells(&mut self, cells: HashSet<u32>) {
        self.grid = cells;
    }

    pub fn move_cells(&mut self) {
        let mut next_grid = HashSet::new();

        for i in 0..=self.cells_cnt() {
            if self.forecast_cell(i) {
                next_grid.insert(i);
            } else {
                next_grid.remove(&i);
            }
        }
        self.set_cells(next_grid);
    }

    fn cell_x(&self, i: u32) -> u32 {
        i % self.width
    }

    fn cell_y(&self, i: u32) -> u32 {
        i / self.height
    }

    pub fn squares(&self) -> Vec<grid::Square> {
        self.grid
            .iter()
            .map(|n| grid::Square::new(self.cell_x(*n), self.width, self.cell_y(*n), self.height))
            .collect::<Vec<grid::Square>>()
    }
}
