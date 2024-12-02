use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

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

/// Helper function that checks if removing any single number makes the sequence safe
fn check_sequence_with_removal(numbers: &[i32]) -> i32 {
    // If it's already safe, no need to try removing numbers
    if check_sequence(numbers) == 1 {
        return 1;
    }
    
    // Try removing each number one at a time
    for i in 0..numbers.len() {
        // Create a new vector without the i-th number
        let mut modified = Vec::with_capacity(numbers.len() - 1);
        modified.extend_from_slice(&numbers[..i]);
        modified.extend_from_slice(&numbers[i + 1..]);
        
        // Check if this modified sequence is safe
        if check_sequence(&modified) == 1 {
            return 1;
        }
    }
    
    0
}

/// Process all sequences and return the count of valid ones
fn part1(lists: &[Vec<i32>]) -> i32 {
    lists.iter()
        .map(|list| check_sequence(list))
        .sum()
}

/// Part 2: Check sequences allowing removal of one number
fn part2(lists: &[Vec<i32>]) -> i32 {
    lists.iter()
        .map(|list| check_sequence_with_removal(list))
        .sum()
}

/// Reads input file and returns a vector of vectors where each inner vector
/// contains integers from one line, padded to match the longest line
fn read_input(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut number_lists: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() { continue; }
        
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;
        
        number_lists.push(numbers);
    }

    Ok(number_lists)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get filename from command line argument, or use "input.txt" as default
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    
    match read_input(&file_path) {
        Ok(lists) => {
            // Print all lists with their results for both parts
            println!("All lists and their results:");
            for (i, list) in lists.iter().enumerate() {
                let result1 = check_sequence(list);
                let result2 = check_sequence_with_removal(list);
                println!("List {:3}: {:?} -> Part1: {}, Part2: {}", 
                    i + 1, list, result1, result2);
            }
            
            // Calculate and print results
            let part1_result = part1(&lists);
            let part2_result = part2(&lists);
            println!("\nPart 1 Result: {}", part1_result);
            println!("Part 2 Result: {}", part2_result);
            
            println!("\nTotal number of lists: {}", lists.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    
    Ok(())
} 