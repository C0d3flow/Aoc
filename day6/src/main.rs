use std::fs::File;
use std::io::{self, BufRead};

fn find_starting_position(grid: &[Vec<char>]) -> Option<(usize, usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if "^>v<".contains(cell) {
                let direction_index = match cell {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                };
                return Some((row_index, col_index, direction_index));
            }
        }
    }
    None 
}

fn calc_next_pos(grid: &mut [Vec<char>], dir: (i32, i32), pos: &mut (usize, usize)) -> (bool, bool) {
    let (row, col) = *pos;
    let mut border_hit = false;
    let mut obstacle_hit = false;
    let next_row = row as i32 + dir.0;
    let next_col = col as i32 + dir.1;

    if next_row < 0
        || next_row >= grid.len() as i32
        || next_col < 0
        || next_col >= grid[0].len() as i32
    {
        border_hit = true;
        println!("Border: Next Col: {} Next Row: {} Max Col: {} Max Row {}", next_col, next_row, grid[0].len(), grid.len());
    } else if grid[next_row as usize][next_col as usize] == '#' {
        obstacle_hit = true;
        println!("Obstacle: Next Col: {} Next Row: {} Max Col: {} Max Row {}", next_col, next_row, grid[0].len(), grid.len());
    } else {
        *pos = (next_row as usize, next_col as usize);
    }

    println!("Calc - Border hit: {}, obstacle hit: {}", border_hit, obstacle_hit);
    (border_hit, obstacle_hit)
}

fn move_till_obstacle(grid: &mut [Vec<char>], dir: (i32, i32), pos: &mut (usize, usize)) -> (u32, bool) {
    let mut visited_count = 0;
    let mut border_hit = false;
    let mut obstacle_hit = false;
    loop {
        if grid[pos.0][pos.1] != 'X' {
            grid[pos.0][pos.1] = 'X';
            visited_count += 1;
        }

        (border_hit, obstacle_hit) = calc_next_pos(grid, dir, pos);
        if (border_hit || obstacle_hit) {
            break;
        }
    }

    println!("Move - Border hit: {}, obstacle hit: {}", border_hit, obstacle_hit);
    (visited_count, border_hit)
}

fn do_tha_moves(grid: &mut [Vec<char>], pos: &mut (usize, usize)) -> u32 {
    let mut visited_count = 0;
    let mut visited_iteration = 0;
    let mut border_hit = false;
    let directions: [(i32, i32); 4] = [
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];
    let mut dir_ptr = 0;
    while !border_hit {
        (visited_iteration, border_hit) = move_till_obstacle(grid, directions[dir_ptr], pos);
        visited_count += visited_iteration;
        dir_ptr += 1;
        if dir_ptr == 4 {
            dir_ptr = 0;
        }
        println!("Direction: {} x: {}, y: {}", dir_ptr, directions[dir_ptr].0, directions[dir_ptr].1);
        println!("Border hit: {}", border_hit);
    }

    visited_count
}

fn check_numbers_in_file(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut grid: Vec<Vec<char>> = reader
       .lines() 
       .filter_map(|line| line.ok()) 
       .map(|line| line.chars().collect()) 
       .collect();

    let mut num_xs = 0;

    if let Some((start_row, start_col, direction_index)) = find_starting_position(&grid) {
        println!(
            "Starting position: ({}, {}), Facing direction index: {}",
            start_row, start_col, direction_index
        );
        num_xs = do_tha_moves(&mut grid, &mut (start_row, start_col));
        print_grid(&grid);
    } else {
        println!("No starting position found!");
    }

    Ok(num_xs)
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!(); // Add a blank line for readability
}


fn main() {
    let filename = "./input.txt";
    match check_numbers_in_file(filename) {
        Ok(num_xs) => {
            println!("Places visited: {:?}", num_xs);
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}