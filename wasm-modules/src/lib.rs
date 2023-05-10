use sha256::digest;
use std::fmt;
use to_binary::BinaryString;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Grid {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Grid {
    pub fn new() -> Grid {
        console_error_panic_hook::set_once();
        let width = 15;
        let height = 15;

        let cells = (0..width * height).map(|_| Cell::Dead).collect();

        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self, seed: &str) {
        let input: String = String::from(seed);
        let val: String = digest(input);
        let x: BinaryString = BinaryString::from(val);
        let mut next: Vec<Cell> = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let id: usize = self.get_index(row, col);
                let next_cell: Cell = match x.0.as_bytes()[id] {
                    b'0' => Cell::Dead,
                    b'1' => Cell::Alive,
                    _ => Cell::Dead,
                };
                next[id] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol: char = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.width + column) as usize
    }
}
