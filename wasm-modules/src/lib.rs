use sha256::digest;
use to_binary::BinaryString;
use wasm_bindgen::prelude::*;

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
    pub fn tick(&mut self, seed: &str) -> String {
        let input = String::from(seed);
        let val = digest(input);
        let x = BinaryString::from(val);
        x.0
    }
}

impl Grid {
    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.width + column) as usize
    }
}
