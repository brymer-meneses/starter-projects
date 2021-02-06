
// Game of Life v1.0
// Brymer Meneses
// Feb. 2, 2021

use std::io;
use std::io::Write;

extern crate ansi_term;
use ansi_term::Colour::{Black, White};

use rand::Rng;

fn initialize_grid(grid: &mut Vec<Vec<usize>>, size_x: &usize, size_y: &usize) -> Vec<Vec<usize>>{
    let state : Vec<usize> = vec![0, 1];

    for i in 0..(*size_y) {
        for j in 0..(*size_x) {

            let random_choice : usize = rand::thread_rng().gen_range(0..2);
            grid[i][j] = state[random_choice];
        }
    };
    grid.to_vec()

}

fn main() {

    let dead_cell = White.on(White).paint(".");
    let alive_cell = Black.on(Black).paint(".");
    
    println!("Game of Life");
    // Get input from user 

    print!("Enter board size (x): ");
    io::stdout()
        .flush()
        .unwrap();

    let mut size_x = String::new();
    io::stdin()
        .read_line(&mut size_x)
        .unwrap();
    
    let size_x : usize = size_x.trim().parse().expect("Error parsing string.");

    print!("Enter board size (y): ");
    io::stdout()
        .flush()
        .unwrap();

    let mut size_y = String::new();
    io::stdin()
        .read_line(&mut size_y)
        .unwrap();
    
    let size_y : usize = size_y.trim().parse().expect("Error parsing string.");
    println!("Board Size: {}x{}", size_x, size_y);

    let mut grid = vec![vec![0; size_x]; size_y];  
    
    // Random Initialization

    let grid = initialize_grid(&mut grid, &size_x, &size_y);

    for i in 0..size_y {
        for j in 0..size_x {
            
            if grid[i][j] == 0 {

                if j % size_x == 0 {
                    print!("\n{}", dead_cell);
                } else {
                    print!("{}", dead_cell); 
                }
            }  else if grid[i][j] == 1 { 
                if j % size_x == 0 {
                    print!("\n{}", alive_cell);
                } else {
                    print!("{}", alive_cell); 
                }
            } 
      }
    } 

}
