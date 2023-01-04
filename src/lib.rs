mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str, location: &str) {
//     alert(
//         &format!("Hello {}, this is {}!", name, location)
//     );
// }

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Alive = 1,
    Dead = 0
}

struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

fn wrapped_neighbor_values(val: u32, size: u32) -> [u32; 3] {
    if val == 0 {
        [size - 1, 0, 1]
    }
    else if val == size - 1 {
        [size - 2, size - 1, 0]
    }
    else {
        [val - 1, val, val + 1]
    }
}

impl World {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn get_cell(&self,row: u32, col: u32) -> Cell {
        self.cells[self.get_index(row, col)]
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let rows = wrapped_neighbor_values(row, self.height);
        let cols = wrapped_neighbor_values(row, self.width);
        let mut count = 0;
        for row2 in rows {
            for col2 in cols {
                if row2 == row && col2 == col {
                    continue;
                }
                let index = (row2 * self.width + col2) as usize;
                count += self.get_cell(row2, col2) as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl World {
    fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let live_count = self.live_neighbor_count(row, col);
                let i = self.get_index(row, col);
                let old_cell_state = self.cells[i];

                next_cells[i] = match (old_cell_state, live_count) {
                    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                    (Cell::Alive, n) if n < 2 => Cell::Dead,
                    // Any live cell with two or three live neighbours lives on to the next generation.
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    // Any live cell with more than three live neighbours dies, as if by overpopulation.
                    (Cell::Alive, n) if n > 3 => Cell::Dead,
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => old_cell_state,
                }
            }
        }
        self.cells = next_cells;
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
