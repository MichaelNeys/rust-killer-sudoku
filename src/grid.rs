use crate::models::SudokuMeta;

// print de 9x9 grid met 3x3 spacing
pub fn print_grid(grid: &[[u8; 9]; 9], invalid_mask: &[[bool; 9]; 9]) {
    for row in 0..9 {
        for col in 0..9 {
            if invalid_mask[row][col] {
                print!("X");
            } else {
                // print waarde als niet leeg
                let value = grid[row][col];
                if value == 0 {
                    print!(".");
                } else {
                    print!("{}", value);
                }
            }
            // spacing
            if col == 2 || col == 5 {
                print!("  ");
            }
        }
        // spacing
        println!();
        if row == 2 || row == 5 {
            println!();
        }
    }
}

// mask helper die invalid cellen markeert
fn check_and_mark( grid: &[[u8; 9]; 9], mask: &mut [[bool; 9]; 9], iter: impl Iterator<Item = (usize, usize)>, target: u32) {
    let cells: Vec<(usize, usize)> = iter.collect();
    let mut sum = 0;
    let mut empty = 0;
    let mut counts = [0u8; 10];
    let mut invalid = false;

    for &(row, col) in &cells {
        let value = grid[row][col];
        if value == 0 {
            empty += 1;
        } else {
            counts[value as usize] += 1;
            if counts[value as usize] > 1 {
                invalid = true;
            }
            sum += value as u32;
        }
    }

    if empty == 0 && sum != target {
        invalid = true;
    }
    if sum > target {
        invalid = true;
    }

    if invalid {
        for &(row, col) in &cells {
            mask[row][col] = true;
        }
    }
}

// helper om te zien welke cellen invalid zijn (voor `Test` command)
pub fn get_invalid_mask(grid: &[[u8; 9]; 9], meta: &SudokuMeta) -> [[bool; 9]; 9] {
    let mut mask = [[false; 9]; 9];
    
    // rijen
    for row in 0..9 {
        check_and_mark(grid, &mut mask, (0..9).map(move |col| (row, col)), 45);
    }
    
    // kolommen
    for col in 0..9 {
        check_and_mark(grid, &mut mask, (0..9).map(move |row| (row, col)), 45);
    }
    
    // blokken
    for block_row in 0..3 {
        for block_col in 0..3 {
            check_and_mark(
                grid,
                &mut mask,
                (0..9).map(move |i| (block_row * 3 + i / 3, block_col * 3 + i % 3)),
                45,
            );
        }
    }

    // check cages
    for cage in &meta.cages {
        let iter = cage.cells.iter().map(|cell| (cell.row, cell.col));
        check_and_mark(grid, &mut mask, iter, cage.sum);
    }

    mask
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

    // lege grid is overal valid
    #[test]
    fn test_get_invalid_mask_empty_grid_is_valid() {
        let grid = [[0u8; 9]; 9];
        let meta = empty_meta();
        let mask = get_invalid_mask(&grid, &meta);

        for row in 0..9 {
            for col in 0..9 {
                assert!(!mask[row][col]);
            }
        }
    }

    // dubbele waarde in rij
    #[test]
    fn test_get_invalid_mask_row_duplicate() {
        let mut grid = [[0u8; 9]; 9];
        grid[0][0] = 5;
        grid[0][1] = 5;
        
        let meta = empty_meta();
        let mask = get_invalid_mask(&grid, &meta);

        for col in 0..9 {
            assert!(mask[0][col]);
        }
    }

    // dubbele waarde in block
    #[test]
    fn test_get_invalid_mask_block_duplicate() {
        let mut grid = [[0u8; 9]; 9];
        grid[0][0] = 3;
        grid[1][1] = 3;
        
        let meta = empty_meta();
        let mask = get_invalid_mask(&grid, &meta);

        assert!(mask[0][0]);
        assert!(mask[1][1]);
        
        // de rest van de block moet ook invalid zijn
        assert!(mask[0][1]);
    }

    #[test]
    fn test_get_invalid_mask_cage_sum_exceeded() {
        let mut grid = [[0u8; 9]; 9];
        let mut meta = empty_meta();

        // cage met 2 cellen en som 5
        meta.cages.push(Cage {
            cells: vec![Cell { row: 5, col: 5 }, Cell { row: 5, col: 6 }],
            sum: 5,
        });

        // met 6 zitten we al boven de som
        grid[5][5] = 6;

        let mask = get_invalid_mask(&grid, &meta);

        // beide zouden nu invalid moeten zijn
        assert!(mask[5][5]);
        assert!(mask[5][6]);
    }

    // dubbele waarde in rij, kolom en blok testen in is_valid_at
    #[test]
    fn test_get_invalid_mask_cage_full_wrong_sum() {
        let mut grid = [[0u8; 9]; 9];
        let mut meta = empty_meta();

        // cage met 2 cellen en som 10
        meta.cages.push(Cage {
            cells: vec![Cell { row: 2, col: 2 }, Cell { row: 2, col: 3 }],
            sum: 10,
        });

        grid[2][2] = 3;
        grid[2][3] = 4;

        let mask = get_invalid_mask(&grid, &meta);

        // cage vol maar som is niet 10 dus beide moeten invalid zijn
        assert!(mask[2][2]);
        assert!(mask[2][3]);
    }
}