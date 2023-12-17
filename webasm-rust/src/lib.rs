extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn prompt(solic_1: &str, name: &str) -> String;
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Dead = 0,
    Alife = 1,
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe{
    pub fn new() -> Universe{
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alife
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32{
        self.width
    }

    pub fn height(&self) -> u32{
        self.height
    }

    pub fn cells(&self) -> *const Cell{
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String{
        self.to_string()
    }

    pub fn tick(&mut self){
        let mut next = self.cells.clone();

        for row in 0..self.height{
            for col in 0..self.width{
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor){
                    // Se uma célula tem menos que dois vizinhos vivos, a célula morre por subpopulação.
                    (Cell::Alife, x) if x < 2 => Cell::Dead,

                    // Se a célula tiver 2 ou 3 vizinhos vivos, a célula vive.
                    (Cell::Alife, 2) | (Cell::Alife, 3) => Cell::Alife,

                    // Se a célula tiver mais que 3 vizinhos, a célula morre por superpopulação.
                    (Cell::Alife, x) if x > 3 => Cell::Dead,

                    // Toda célula morta com exatos 3 vizinhos vivos, se torna viva, viva a reprodução!!!
                    (Cell::Dead, 3) => Cell::Alife,

                    // Todas as outras células recebem o mesmo estado.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize{
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8{
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned(){
            for delta_column in [self.width - 1, 0, 1].iter().cloned(){
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column +  delta_column) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_column);
                count += self.cells[idx] as u8;
            }
        }

        count
    }
}

use std::fmt;

impl fmt::Display for Universe{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize){
            for &cell in line{
                let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}