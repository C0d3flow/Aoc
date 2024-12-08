use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Initialize data structures
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // Read the file
    let path = "./input.txt"; 
    let input = File::open(path)?;
    let reader = io::BufReader::new(input);
    let mut is_rule_section = true;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            // Empty line marks transition to updates
            is_rule_section = false;
            continue;
        }

        if is_rule_section {
            // Parse rules
            let parts: Vec<&str> = line.split('|').collect();
            if let (Some(&x), Some(&y)) = (parts.get(0), parts.get(1)) {
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                rules.entry(x).or_insert(Vec::new()).push(y);
            }
        } else {
            // Parse updates
            let update: Vec<i32> = line.split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    let mut sum_of_middle_pages = 0;
    let mut correct_lines = 0;
    let mut incorrect_lines: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut correct_line = true;
        for (i, &page) in update.iter().enumerate() {
            // Check if there are any rules for the current page
            if let Some(dependent_pages) = rules.get(&page) {
                for &dependent_page in dependent_pages {
                    // Ensure `dependent_page` appears **after** `page` in the current update
                    if let Some(pos) = update.iter().position(|&p| p == dependent_page) {
                        if pos < i {
                            correct_line = false;
                            incorrect_lines.push(update.clone());
                            break;
                        }
                    }
                }
            }
            if !correct_line {
                break;
            }
        }
        if correct_line {
            sum_of_middle_pages += update[update.len()/2];
            correct_lines += 1;
        }
    }

    println!("Correct updates: {}", correct_lines);
    println!("Sum of middle pages in correct updates: {}", sum_of_middle_pages);

    Ok(())
}
