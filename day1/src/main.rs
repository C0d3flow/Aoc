use std::fs::File;
use std::io::{self, BufRead};


fn read_numbers_into_columns(filename: &str) -> io::Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?; // Result auspacken
        if let Some((first, second)) = parse_line_to_tuple(&line) {
            column1.push(first);
            column2.push(second);
        }
    }

    column1.sort();
    column2.sort();

    Ok((column1, column2))
}

fn parse_line_to_tuple(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        if let (Ok(first), Ok(second)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            return Some((first, second));
        }
    }
    None
}

fn get_tuples_from_file(filename: &str) -> io::Result<Vec<(u32, u32)>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut numbers = Vec::new();
    for line in reader.lines() {
        let line = line?; 
        if let Some((first, second)) = parse_line_to_tuple(&line) {
            numbers.push((first, second));
        }
    }

    Ok(numbers)
}

fn sum_of_absolute_differences(numbers: &[(u32, u32)]) -> u32 {
    numbers.iter()
        .map(|(a, b)| ((*a as i64 - *b as i64)).abs() as u32) 
        .sum() 
}

fn main() {
    let filename = "C:/git/AoC/hello_world/input.txt";
    match get_tuples_from_file(filename) {
        Ok(numbers) => {
            println!("Sum of distances: {:?}", sum_of_absolute_differences(&numbers));
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
    match read_numbers_into_columns(filename) {
        Ok((column1, column2)) => {
            let combined: Vec<_> = column1.into_iter().zip(column2.into_iter()).collect();
            println!("Sum of sorted distances: {:?}", sum_of_absolute_differences(&combined));
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}
