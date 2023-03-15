mod utils;

use wasm_bindgen::prelude::*;
use Direction::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    White,
    Black,
}

#[wasm_bindgen]
pub struct Ant {
    x: u32,
    y: u32,
    direction: Direction, // direction that ant is facing
}

#[wasm_bindgen]
impl Ant {
    fn new() -> Ant {
        Ant {
            x: 8,
            y: 8,
            direction: Direction::West,
        }
    }

    fn get_x(&self) -> u32 {
        self.x
    }

    fn get_y(&self) -> u32 {
        self.y
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Direction {
    East,
    West,
    South,
    North,
}

#[wasm_bindgen]
impl Direction {
    fn left(&self) -> Direction {
        match self {
            East => North,
            West => South,
            North => West,
            South => East,
        }
    }

    fn right(&self) -> Direction {
        match self {
            East => South,
            West => North,
            North => East,
            South => West,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    ant: Ant,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn move_ant(&mut self) {
        let mut new = self.cells.clone();
        let row = self.ant.get_x();
        let col = self.ant.get_y();
        let index = self.get_index(col, row);
        let mut cell = self.cells[index];
        // change direction and colour to white/black
        match cell {
            Cell::White => {
                self.ant.direction = self.ant.direction.right();
                cell = Cell::Black;
            },
            Cell::Black => {
                self.ant.direction = self.ant.direction.left();
                cell = Cell::White;
            },
        }
        // move direction
        match self.ant.direction {
            East => self.ant.x += 1,
            West => self.ant.x -= 1,
            South => self.ant.y += 1,
            North => self.ant.y -= 1,
        }
        new[index] = cell;

        self.cells = new;
    }

    pub fn new() -> Universe {
        let width = 17;
        let height = 17;
        let cells = (0..width * height).map(|_| Cell::White).collect();
        let ant = Ant::new();

        Universe {
            width,
            height,
            cells,
            ant,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_direction(&self) -> Direction {
        self.ant.direction
    }

    pub fn get_x(&self) -> u32 {
        self.ant.get_x()
    }

    pub fn get_y(&self) -> u32 {
        self.ant.get_y()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::White { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
