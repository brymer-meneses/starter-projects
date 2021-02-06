
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

fn count_neighbors(grid: &Vec<Vec<usize>>, index_x: i16, index_y: i16) -> i16 {
    
    // Counts the number of alive neighbors of a cell
    // x x o 
    // o c x 
    // o o x
    let mut alive_neightbors = 0;

    for i in -1..1 {
        for j in -1..1 {
            
            if grid[(index_y + i) as usize][(index_x + j) as usize] == 1 {
                alive_neightbors += 1;
            }
        }
    };


    alive_neightbors - 1


}

fn next_iter(grid: &mut Vec<Vec<usize>>, size_x: usize, size_y: usize) {

    for i in 0..size_y {
        for j in 0..size_x {
            let total_neighbors = count_neighbors(&grid, j as i16, i as i16);
            
            if grid[i][j] == 1 {
                
            }
        }
    }
}

fn display_grid(grid: &Vec<Vec<usize>>, size_x: usize, size_y: usize) {

    let dead_cell = White.on(White).paint(".");
    let alive_cell = Black.on(Black).paint(".");


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

fn main() {

    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    
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
    grid = initialize_grid(&mut grid, &size_x, &size_y);

    display_grid(&grid, size_x, size_y);
}
