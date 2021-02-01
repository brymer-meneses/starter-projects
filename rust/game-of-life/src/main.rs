
// Game of Life v1.0
// Brymer Meneses
// Feb. 2, 2021

use std::io;
use std::io::Write;

extern crate ansi_term;
use ansi_term::Colour::{Black, White};

use rand::Rng;

fn initialize_grid(grid: &mut Vec<Vec<usize>>, size: usize) -> Vec<Vec<usize>>{
    let state = vec![0, 1];
    let random_choice : usize = rand::thread_rng().gen_range(0..2);

    for i in 0..size {
        for j in 0..size {
            grid[i][j] = state[random_choice];
        }
    };
    grid.to_vec()

}

fn main() {

    let dead_cell = White.on(White).paint("0");
    let alive_cell = Black.on(Black).paint("1");
    
    println!("Game of Life");
    // Get input from user 

    print!("Enter board size: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .unwrap();
    
    let size : usize = size.trim().parse().expect("Error parsing string.");

    println!("Board Size: {}x{}", size, size);

    let mut grid = vec![vec![0; size]; size];  
    
    // Random Initialization

    let grid = initialize_grid(&mut grid, size);

    for i in 0..size {
        for j in 0..size {
            
            if grid[i][j] == 0 {

                if j % size == 0 {
                    print!("\n{}", dead_cell);
                } else {
                    print!("{}", dead_cell); 
                }
            }
      }
    } 

}
