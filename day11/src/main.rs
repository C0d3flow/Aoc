use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn read_stones_from_file(filename: &str) -> io::Result<HashMap<u64, u64>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut stones = HashMap::new();
    if let Some(Ok(line)) = reader.lines().next() {
        for value in line.split_whitespace() {
            if let Ok(stone) = value.parse::<u64>() {
                *stones.entry(stone).or_insert(0) += 1;
            }
        }
    }

    Ok(stones)
}

fn blink_new(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();

    for (stone, count) in stones {
        if count == 0 {
            continue; // Skip stones with zero count
        }

        let stone_str = stone.to_string();
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += count;
        } else if stone_str.len() % 2 == 0 {
            let base: u64 = 10;
            let power: u32 = (stone_str.len() as u32 / 2).try_into().unwrap();
            let left = stone / base.pow(power);
            let right = stone % base.pow(power);

            *new_stones.entry(left).or_insert(0) += count;
            *new_stones.entry(right).or_insert(0) += count;
        } else {
            let new_stone = stone * 2024;
            *new_stones.entry(new_stone).or_insert(0) += count;
        }
    }

    new_stones
}

fn count_stones(stones: HashMap<u64, u64>) -> u64 {
    let mut sum: u64 = 0;
    for (_, count) in stones {
        sum += count as u64;
    }
    sum
}

fn print_stones(stones: HashMap<u64, u64>) {
    for (stone, count) in stones {
        println!("Stone {} Count {}", stone, count);
    }
}

fn run_evolution_for_x_blinks(x: u32, mut stones: HashMap<u64, u64>) -> u64 {
    let mut num_stones = 0;
    for i in 0..x {
        stones = blink_new(stones.clone());
        num_stones = count_stones(stones.clone());
        println!("Day {} Stones {}", i, num_stones);
        //print_stones(stones.clone());
    }
    num_stones
}

fn main() {
    let filename = "./input.txt";
    match read_stones_from_file(filename) {
        Ok(stones) => {
            println!("Num Stones: {:?}", run_evolution_for_x_blinks(75, stones));
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}