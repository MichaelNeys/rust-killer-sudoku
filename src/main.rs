mod grid;
mod models;
mod solver;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use grid::{get_invalid_mask, print_grid};
use models::{Given, SudokuJson, SudokuMeta};
use solver::solve_recursive;

#[derive(Parser)]
struct Cli {
    /// JSON bestand van de Killer Sudoku
    sudoku: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// print de Killer Sudoku
    Print,
    /// test of de Killer Sudoku valid is
    Test,
    /// los de Killer Sudoku op
    Solve {
        /// output file voor de oplossing
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// single threaded oplossen
        #[arg(long)]
        seq: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // verwerk JSON
    let file = File::open(&cli.sudoku).expect("Kan het JSON bestand niet openen");
    let reader = BufReader::new(file);
    let original_json: SudokuJson = serde_json::from_reader(reader).expect("Kon JSON niet parsen");

    // init grid
    let mut grid = [[0u8; 9]; 9];
    for given in &original_json.given {
        grid[given.row][given.col] = given.value;
    }

    // init meta
    let mut cell_to_cage = [[usize::MAX; 9]; 9];
    for (cage_index, cage) in original_json.cages.iter().enumerate() {
        for cell in &cage.cells {
            cell_to_cage[cell.row][cell.col] = cage_index;
        }
    }
    let meta = SudokuMeta {
        cages: original_json.cages.clone(),
        cell_to_cage,
    };

    match cli.command {
        Commands::Print => {
            let mask = [[false; 9]; 9];
            print_grid(&grid, &mask);
        }
        Commands::Test => {
            let mask = get_invalid_mask(&grid, &meta);
            print_grid(&grid, &mask);
        }
        Commands::Solve { output, seq } => {            
            if let Some(solution) = solve_recursive(&grid, &meta, 0, seq) {
                let mask = [[false; 9]; 9];
                print_grid(&solution, &mask);

                // als output gegeven is proberen we te schrijven naar JSON
                if let Some(out_path) = output {
                    let mut out_json = original_json.clone();
                    out_json.given.clear();
                    for (r, row) in solution.iter().enumerate() {
                        for (c, &value) in row.iter().enumerate() {
                            out_json.given.push(Given {
                                row: r,
                                col: c,
                                value,
                            });
                        }
                    }
                    let out_file = File::create(out_path).expect("Could not create output file");
                    serde_json::to_writer_pretty(out_file, &out_json)
                        .expect("Could not write solution to JSON");
                }
            } else {
                // originele als we geen oplossing hebben
                let mask = [[false; 9]; 9];
                print_grid(&grid, &mask);
            }
        }
    }
}