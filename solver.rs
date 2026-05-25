use crate::models::SudokuMeta;
use rayon::prelude::*;

// nagaan of een zone in de sudoku valid is
fn check_house(grid: &[[u8; 9]; 9], iter: impl Iterator<Item = (usize, usize)>, target: u32) -> bool {
    let mut sum = 0;
    let mut empty = 0;
    let mut counts = [0u8; 10];

    for (row, col) in iter {
        let value = grid[row][col];
        if value == 0 {
            empty += 1;
        } else {
            counts[value as usize] += 1;
            if counts[value as usize] > 1 {
                return false;
            }
            sum += value as u32;
        }
    }
    
    // standaard checks
    if sum > target {
        return false;
    }
    if empty == 0 && sum != target {
        return false;
    }

    // min en max bepalen voor de lege cellen
    if empty > 0 {
        let mut min_possible = sum;
        let mut max_possible = sum;
        
        let mut min_val = 1;
        let mut max_val = 9;
        
        for _ in 0..empty {
            // zoek kleinste niet gebruikte getal
            while min_val <= 9 && counts[min_val as usize] > 0 { min_val += 1; }
            if min_val <= 9 {
                min_possible += min_val as u32;
                min_val += 1;
            }
            
            // zoek grootste niet gebruikte getal
            while max_val >= 1 && counts[max_val as usize] > 0 { max_val -= 1; }
            if max_val >= 1 {
                max_possible += max_val as u32;
                max_val -= 1;
            }
        }

        // als max mogelijk nie haalt of min mogelijk te veel is stoppen we
        if max_possible < target || min_possible > target {
            return false;
        }
    }

    true
}

// checken of een bepaalde cell valid is in de sudoku
fn is_valid_at(grid: &[[u8; 9]; 9], meta: &SudokuMeta, row: usize, col: usize) -> bool {
    // rijen
    if !check_house(grid, (0..9).map(|col| (row, col)), 45) { return false; }
    // kolommen
    if !check_house(grid, (0..9).map(|row| (row, col)), 45) { return false; }
    // blokken
    let block_row = row / 3 * 3;
    let block_col = col / 3 * 3;
    if !check_house(grid, (0..9).map(|i| (block_row + i / 3, block_col + i % 3)), 45) { return false; }
    
    // cages
    let cage_idx = meta.cell_to_cage[row][col];
    if cage_idx != usize::MAX {
        let cage = &meta.cages[cage_idx];
        if !check_house(grid, cage.cells.iter().map(|cell| (cell.row, cell.col)), cage.sum) {
            return false;
        }
    }
    true
}

// recursief oplossen met grid en meta van onze sudoku
pub fn solve_recursive(grid: &[[u8; 9]; 9], meta: &SudokuMeta, depth: usize, seq: bool) -> Option<[[u8; 9]; 9]> {
    
    // zoek lege cel met minste opties
    let mut best_cell = None;
    let mut min_options = 10;
    let mut best_valid_moves = Vec::new();

    for row in 0..9 {
        for col in 0..9 {
            if grid[row][col] == 0 {
                let mut valid_moves = Vec::new();
                for value in 1..10 {
                    let mut test_grid = *grid;
                    test_grid[row][col] = value;
                    if is_valid_at(&test_grid, meta, row, col) {
                        valid_moves.push(value);
                    }
                }
                
                // stoppen als er geen valid move is
                let options = valid_moves.len();
                if options == 0 {
                    return None; 
                }

                if options < min_options {
                    min_options = options;
                    best_cell = Some((row, col));
                    best_valid_moves = valid_moves;
                    
                    // niet verder zoeken als er maar 1 optie is
                    if min_options == 1 {
                        break;
                    }
                }
            }
        }
        if min_options == 1 { break; }
    }

    // oplossing gevonden als we geen lege cellen meer hebben
    let Some((row, col)) = best_cell else {
        return Some(*grid);
    };

    // closure om een waarde te proberen en recursief verder te gaan
    let try_value = |value: u8| -> Option<[[u8; 9]; 9]> {
        let mut new_grid = *grid;
        new_grid[row][col] = value;
        solve_recursive(&new_grid, meta, depth + 1, seq)
    };

    // eerste 4 levels parallel tenzij --seq
    if !seq && depth < 4 {
        best_valid_moves.into_par_iter().find_map_any(try_value)
    } else {
        best_valid_moves.into_iter().find_map(try_value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Cage, Cell};

    fn empty_meta() -> SudokuMeta {
        SudokuMeta {
            cages: vec![],
            cell_to_cage: [[usize::MAX; 9]; 9],
        }
    }

    // 2 dezelfde getallen mogen niet voorkomen
    #[test]
    fn test_check_house_fast_duplicates() {
        let mut grid = [[0u8; 9]; 9];
        grid[0][0] = 5;
        grid[0][1] = 5;
        
        let is_valid = check_house(&grid, (0..2).map(|col| (0, col)), 10);
        assert!(!is_valid);
    }

    // de som van de getallen mag niet groter zijn dan de target
    #[test]
    fn test_check_house_fast_sum_exceeded() {
        let mut grid = [[0u8; 9]; 9];
        grid[0][0] = 8;
        grid[0][1] = 9;
        
        let is_valid = check_house(&grid, (0..3).map(|col| (0, col)), 15);
        assert!(!is_valid);
    }

    // min max testen
    #[test]
    fn test_check_house_fast_min_max() {
        let mut grid = [[0u8; 9]; 9];
        grid[0][0] = 1;
        grid[0][1] = 2;
        
        // 12 < 15 moet falen dus (grootst mogelijke over is 9)
        let is_valid_max = check_house(&grid, (0..3).map(|col| (0, col)), 15);
        assert!(!is_valid_max);

        // 6 > 4 moet ook falen (kleinst mogelijke over is 3)
        let is_valid_min = check_house(&grid, (0..3).map(|col| (0, col)), 4);
        assert!(!is_valid_min);
    }

    // test dat is_valid_at de juiste checks doet
    #[test]
    fn test_is_valid_at() {
        let mut grid = [[0u8; 9]; 9];
        let meta = empty_meta();
        
        // rij check
        grid[0][0] = 1;
        grid[0][8] = 1; 
        assert!(!is_valid_at(&grid, &meta, 0, 8));

        // kolom check
        grid[6][8] = 1;
        grid[7][8] = 1; 
        assert!(!is_valid_at(&grid, &meta, 7, 8));

        // 3x3 check
        grid[3][0] = 1;
        grid[3][1] = 1; 
        assert!(!is_valid_at(&grid, &meta, 3, 1));
    }

    // cages testen in is_valid_at
    #[test]
    fn test_is_valid_at_with_cage() {
        let mut grid = [[0u8; 9]; 9];
        let mut meta = empty_meta();
        
        // cage met 3 cellen en som 10
        meta.cages.push(Cage {
            cells: vec![
                Cell { row: 0, col: 0 },
                Cell { row: 0, col: 1 },
                Cell { row: 0, col: 2 },
            ],
            sum: 10,
        });
        // Update de naslag-tabel (cel behoort tot cage index 0)
        meta.cell_to_cage[0][0] = 0;
        meta.cell_to_cage[0][1] = 0;
        meta.cell_to_cage[0][2] = 0;

        // 9 op eerste plaats
        grid[0][0] = 9;
        // 2 op tweede plaats is te groot dus zou moeten falen bij is_valid_at
        grid[0][1] = 2; 
        
        assert!(!is_valid_at(&grid, &meta, 0, 1));
    }
}