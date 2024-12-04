use std::fs::File;
use std::io::{self, BufRead};

fn parse_line_to_vector(line: &str) -> Vec<u32> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts
        .into_iter() 
        .filter_map(|s| s.parse::<u32>().ok()) 
        .collect()
}


fn check_numbers_in_file(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut good_codes = 0;

    for line in reader.lines() {
        let line = line?;
        let code = parse_line_to_vector(&line);

        let is_ascending = code
            .windows(2)
            .all(|pair| pair[1] > pair[0] && (pair[1] - pair[0]) < 4);

        let is_descending = code
            .windows(2)
            .all(|pair| pair[0] > pair[1] && (pair[0] - pair[1]) < 4);

        if is_ascending || is_descending {
            good_codes += 1;
        }
    }
    Ok(good_codes)
}

fn main() {
    let filename = "./input.txt";
    match check_numbers_in_file(filename) {
        Ok(good_codes) => {
            println!("Good codes: {:?}", good_codes);
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}
