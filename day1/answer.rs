use std::fs::File;
use std::io::{self, BufReader, BufRead};

/// Calculates the sum of absolute differences between corresponding elements of two vectors
fn sum_absolute_differences(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1.iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

/// Sorts a vector of integers in ascending order
fn sort_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut sorted = numbers.clone();
    sorted.sort();
    sorted
}

/// Reads a file containing two columns of numbers and returns them as separate vectors
fn read_input(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        if numbers.len() == 2 {
            first_numbers.push(numbers[0].parse::<i32>()?);
            second_numbers.push(numbers[1].parse::<i32>()?);
        }
    }
    
    Ok((first_numbers, second_numbers))
}

/// Part 1: Calculate and compare sums of absolute differences between original and sorted lists
fn part1(first_numbers: &Vec<i32>, second_numbers: &Vec<i32>) {
    // Sort both vectors
    let sorted_first = sort_numbers(first_numbers);
    let sorted_second = sort_numbers(second_numbers);
    
    // Calculate sum of absolute differences for both original and sorted lists
    let original_diff_sum = sum_absolute_differences(first_numbers, second_numbers);
    let sorted_diff_sum = sum_absolute_differences(&sorted_first, &sorted_second);
    
    // Print results
    println!("Original first column: {:?}", first_numbers);
    println!("Sorted first column: {:?}", sorted_first);
    println!("\nOriginal second column: {:?}", second_numbers);
    println!("Sorted second column: {:?}", sorted_second);
    
    println!("\nSum of absolute differences in original lists: {}", original_diff_sum);
    println!("Sum of absolute differences in sorted lists: {}", sorted_diff_sum);
}

/// Part 2: Calculate similarity score between two lists
/// For each number in first list, multiply it by how many times it appears in second list
fn part2(first_numbers: &Vec<i32>, second_numbers: &Vec<i32>) -> i32 {
    first_numbers.iter()
        .map(|&num| {
            // Count occurrences of current number in second list
            let occurrences = second_numbers.iter()
                .filter(|&&n| n == num)
                .count() as i32;
            // Multiply the number by how many times it appears
            num * occurrences
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input";
    
    match read_input(file_path) {
        Ok((first_numbers, second_numbers)) => {
            println!("Part 1 Results:");
            println!("---------------");
            part1(&first_numbers, &second_numbers);
            
            println!("\nPart 2 Results:");
            println!("---------------");
            let total_matches = part2(&first_numbers, &second_numbers);
            println!("Total number of matches: {}", total_matches);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    
    Ok(())
}

