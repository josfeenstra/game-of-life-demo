use std::{fmt, usize};
use wasm_bindgen::prelude::*;

extern crate js_sys;

#[wasm_bindgen] 
#[repr(u8)] // this forces the representation to be u8'ts 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen] 
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn get(&self, row: u32, col: u32) -> Cell {
        return self.cells[self.get_index(row, col)];
    }

    fn get_index(&self, row: u32, col:u32) -> usize {
        (self.width * row + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // instead of -1, we do +length-1, 
        // with modulus it loops back to mean the same thing
        for drow in [self.width-1, 0, 1].iter().cloned() {
            for dcol in [self.height-1,0,1].iter().cloned() {

                if drow == 0 && dcol == 0 {
                    continue;
                }

                let i = (row + drow) % self.height;
                let j = (col + dcol) % self.width;
                count += self.get(i, j) as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {

        // wow, you can do javascript-like syntax in rust, wow
        // no idea what || means tho
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn update(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {

                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let nbs_count = self.live_neighbour_count(row, col);
                let next_cell = match (cell, nbs_count) {

                    (Cell::Alive, x) if x < 2 => Cell::Dead,            // underpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,            // overpopulation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, // stable
                    (Cell::Dead, 3) => Cell::Alive,                     // reproduction
                    (value, _) => value,                                // nothing
                
                };
                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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