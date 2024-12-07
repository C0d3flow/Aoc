use std::fs::File;
use std::io::{self, BufRead};

fn check_numbers_in_file(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
       .lines() 
       .filter_map(|line| line.ok()) 
       .map(|line| line.chars().collect()) 
       .collect();

    let mut num_xmas = 0;

    let directions: [(i32, i32); 8] = [
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
        (-1, 0), // up
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1),// up-left
    ];

    let chars = [ 'X', 'M', 'A', 'S' ];
    let nums = [ 1,2,3 ];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == chars[0] {
                //print!("X");
                for dir in directions {
                    for i in nums {
                        let x = row_idx as i32 + i * dir.0;
                        let y = col_idx as i32 + i * dir.1;
                        if (x >= 0) && (x < grid.len() as i32) && (y >= 0) && (y < row.len() as i32) {
                            if grid[x as usize][y as usize] != chars[i as usize] {
                                break;
                            }
                            else if i == 3 {
                                num_xmas += 1;
                                break;
                            }
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(num_xmas)
}

fn main() {
    let filename = "./input.txt";
    match check_numbers_in_file(filename) {
        Ok(xmas) => {
            println!("XMAS appearences: {:?}", xmas);
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}
