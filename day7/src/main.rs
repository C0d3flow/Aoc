use std::fs::File;
use std::io::{self, BufRead};

fn generate_operator_combinations(n: usize) -> Vec<Vec<char>> {
    let operators = ['+', '*', '|']; 
    let mut combinations = vec![];

    for i in 0..3usize.pow(n as u32) {
        let mut combo = vec![];
        let mut value = i;

        for _ in 0..n {
            combo.push(operators[value % 3]);
            value /= 3;
        }
        combinations.push(combo);
    }

    combinations
}

fn evaluate_expression(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];

    for (i, &operator) in operators.iter().enumerate() {
        match operator {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                let concat_value = format!("{}{}", result, numbers[i + 1]).parse::<i64>().unwrap();
                result = concat_value;
            }
            _ => panic!("Unknown operator"),
        }
    }

    result
}



fn check_numbers_in_file(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    

    let mut sum = 0;    

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
            let test_value: i64 = parts[0].parse().unwrap();
            let numbers: Vec<i64> = parts[1].split_whitespace().map(|n| n.parse().unwrap()).collect();

            let num_operators = numbers.len() - 1;
            let operator_combinations = generate_operator_combinations(num_operators);

            let mut is_valid = false;

            for operators in operator_combinations {
                let result = evaluate_expression(&numbers, &operators);
                if result == test_value {
                    is_valid = true;
                    break;
                }
            }

            if is_valid {
                sum += test_value;
            }
        } else {
            eprintln!("Error reading line");
        }
    }

    Ok(sum)
}


fn main() {
    let filename = "./input.txt";
    match check_numbers_in_file(filename) {
        Ok(sum) => {
            println!("Total sum of calibration values: {:?}", sum);
        }
        Err(e) => {
            eprintln!("Error while reading file: {}", e);
        }
    }
}