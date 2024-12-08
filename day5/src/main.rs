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
            let parts: Vec<&str> = line.split('|').collect();
            if let (Some(&x), Some(&y)) = (parts.get(0), parts.get(1)) {
                let x: i32 = x.parse().unwrap();
                let y: i32 = y.parse().unwrap();
                rules.entry(y).or_insert(Vec::new()).push(x);
            }
        } else {
            // Parse updates
            let update: Vec<i32> = line.split(',')
                .map(|num| num.trim().parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    // Check updates for correctness
    let mut correct_lines = 0;
    let mut sum_of_correct_middle_pages = 0;
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    let mut incorrect_lines: Vec<i32> = Vec::new(); 
    let mut line_number = 0;

    for update in &updates {
        let mut correct_line = true;

        for (i, &page) in update.iter().enumerate() {
            // Check if there are any dependencies for the current page
            if let Some(required_pages) = rules.get(&page) {
                for &required_page in required_pages {
                    // Ensure `required_page` appears **before** `page` in the current update
                    if let Some(pos) = update.iter().position(|&p| p == required_page) {
                        if pos > i {
                            correct_line = false;
                            incorrect_updates.push(update.clone());
                            incorrect_lines.push(line_number);
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
            correct_lines += 1;
            sum_of_correct_middle_pages += update[update.len()/2];
        }
        line_number+=1;
    }

    let mut current_line = 0;
    let mut sum_of_sorted_middle_pages = 0;
    for update in &mut incorrect_updates {
        print!("Line {} ", incorrect_lines[current_line]);
        let mut i = 0;
        while i < update.len() {
            let page = update[i]; // Current page
            if let Some(required_pages) = rules.get(&page) {
                let mut swapped = false;
                for &required_page in required_pages {
                    if let Some(pos) = update.iter().position(|&p| p == required_page) {
                        if pos > i {
                            // Swap the pages
                            update.swap(i, pos);
                            print!(" Swapped {} {}", i, pos);
                            swapped = true;
                            break;
                        }
                    }
                }
                if swapped {
                    // Re-check the current index
                    if i > 0 {
                        i -= 1;
                    }
                    continue;
                }
            }
            i += 1; // Move to the next index only if no swap occurred
        }
        println!("");
        // Calculate middle page after sorting
        sum_of_sorted_middle_pages += update[update.len() / 2];
        current_line += 1;
    }


    println!("Correct updates: {}", correct_lines);
    println!("Sum of correct middle pages: {}", sum_of_correct_middle_pages);
    println!("Sum of sorted middle pages: {}", sum_of_sorted_middle_pages);

    Ok(())
}
