use std::fs::File;
use std::io::{self, BufReader, BufRead};

/// Checks if a list meets two conditions:
/// 1. Numbers are either all increasing or all decreasing
/// 2. Difference between adjacent numbers is between 1 and 3 inclusive
fn check_sequence(numbers: &[i32]) -> i32 {
    if numbers.len() < 2 {
        return 0;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;
    
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i-1];
        
        if diff.abs() < 1 || diff.abs() > 3 {
            return 0;
        }
        
        if diff > 0 {
            is_decreasing = false;
        } else {
            is_increasing = false;
        }
        
        if !is_increasing && !is_decreasing {
            return 0;
        }
    }
    
    1
}

/// Process all sequences and return the count of valid ones
fn part1(lists: &[Vec<i32>]) -> i32 {
    lists.iter()
        .map(|list| check_sequence(list))
        .sum()
}

/// Reads input file and returns a vector of vectors where each inner vector
/// contains integers from one line, padded to match the longest line
fn read_input(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut number_lists: Vec<Vec<i32>> = Vec::new();
    let mut max_length = 0;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;
        
        max_length = max_length.max(numbers.len());
        number_lists.push(numbers);
    }

    for list in &mut number_lists {
        if list.len() < max_length {
            let last_value = *list.last().unwrap_or(&0);
            list.resize(max_length, last_value);
        }
    }

    Ok(number_lists)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input.txt";
    
    match read_input(file_path) {
        Ok(lists) => {
            // Print all lists with their results
            println!("All lists and their results:");
            for (i, list) in lists.iter().enumerate() {
                let result = check_sequence(list);
                println!("List {:3}: {:?} -> {}", i + 1, list, result);
            }
            
            // Calculate and print part 1 result
            let part1_result = part1(&lists);
            println!("\nPart 1 Result: {}", part1_result);
            
            println!("\nTotal number of lists: {}", lists.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    
    Ok(())
}
