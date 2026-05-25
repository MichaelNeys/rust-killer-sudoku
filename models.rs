use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Given {
    pub row: usize,
    pub col: usize,
    pub value: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cage {
    pub cells: Vec<Cell>,
    pub sum: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SudokuJson {
    pub given: Vec<Given>,
    pub cages: Vec<Cage>,
}

pub struct SudokuMeta {
    pub cages: Vec<Cage>,
    pub cell_to_cage: [[usize; 9]; 9],
}