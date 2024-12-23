use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn calc_num_antinodes(antinodes: [[u32; 50]; 50]) -> u32 {
    let mut sum = 0;
    for row in antinodes {
        for col in row {
            sum  += col;
            print!("{}",col);
        }
        println!();
    }
    sum
}

fn place_antinodes_for_frequency(antenna_locations: &Vec<(u32, u32)>, antinodes: &mut [[u32; 50]; 50]) {
    for i in 0..antenna_locations.len() {
        for j in i + 1..antenna_locations.len() {
            let mut dir1_done = false;
            let mut dir2_done = false;
            for mul in 0..50 {
                let pair1 = &antenna_locations[i];
                let pair2 = &antenna_locations[j];
                if !dir1_done {
                    let antinode_location_1_x = mul * (pair1.0 as i32 - pair2.0 as i32) + pair1.0 as i32;
                    let antinode_location_1_y = mul * (pair1.1 as i32 - pair2.1 as i32) + pair1.1 as i32;
                    if 0 <= antinode_location_1_x && antinode_location_1_x < 50 && 0 <= antinode_location_1_y  && antinode_location_1_y < 50 {
                        antinodes[antinode_location_1_x as usize][antinode_location_1_y as usize] = 1;
                        //println!("Antenna Position: X: {} Y: {}", antinode_location_1_x, antinode_location_1_y);
                    } else {
                        dir1_done = true;
                    }
                }

                if !dir2_done {
                    let antinode_location_2_x = mul * (pair2.0 as i32- pair1.0 as i32) + pair2.0 as i32;
                    let antinode_location_2_y = mul * (pair2.1 as i32- pair1.1 as i32) + pair2.1 as i32;
                    if 0 <= antinode_location_2_x && antinode_location_2_x < 50 && 0 <= antinode_location_2_y  && antinode_location_2_y < 50 {
                        antinodes[antinode_location_2_x as usize][antinode_location_2_y as usize] = 1;
                        //println!("Antenna Position: X: {} Y: {}", antinode_location_2_x, antinode_location_2_y);
                    } else {
                        dir2_done = true;
                    }
                }
                if dir1_done && dir2_done {
                    break;
                }
            }
        }
    }
}

fn get_frequency_locations(grid: Vec<Vec<char>>) ->  HashMap<char, Vec<(u32, u32)>> {
    let mut frequency_locations: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &symbol) in row.iter().enumerate() {
            frequency_locations
                .entry(symbol)
                .or_insert_with(Vec::new)
                .push((row_index as u32, col_index as u32));
        }
    }
    frequency_locations
}

fn check_grid(filename: &str) -> io::Result<u32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
       .lines() 
       .filter_map(|line| line.ok()) 
       .map(|line| line.chars().collect()) 
       .collect();

    let frequency_locations = get_frequency_locations(grid);

    let mut antinodes: [[u32; 50]; 50] = [[0; 50]; 50];
    for frequency in frequency_locations {
        if '.' == frequency.0 {
            continue;
        }
        place_antinodes_for_frequency(&frequency.1, &mut antinodes);
        if prints < 2 {
            calc_num_antinodes(antinodes);
        }
    }
    let result = calc_num_antinodes(antinodes);

    Ok(result)
}

fn main() {
    let filename = "./input.txt";
    match check_grid(filename) {
        Ok(num_antinodes) => {
            println!("Num Antinodes: {:?}", num_antinodes);
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}