use std::fs::File;
use std::io::Read;
use regex::Regex;

fn sum_of_multiplications(input: &str) -> u32 {
    // This regex pattern matches exactly:
    // - "mul" followed by "("
    // - First number: 1-3 digits
    // - Comma
    // - Second number: 1-3 digits
    // - Closing parenthesis
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    pattern
        .captures_iter(input)  // Find all non-overlapping matches
        .map(|cap| {
            // Extract and multiply the two captured numbers
            let first_num = cap[1].parse::<u32>().unwrap();
            let second_num = cap[2].parse::<u32>().unwrap();
            first_num * second_num
        })
        .sum()  // Add up all multiplication results
}

fn normalize(input: &str) -> String {
    let mut pending = input;
    let mut ignoring = false;
    let mut normalized = String::new();

    loop {
        if ignoring {
            let Some(dox) = pending.find("do()") else {
                break;
            };

            pending = &pending[dox..];
            ignoring = false;
        } else {
            let Some(dont) = pending.find("don't()") else {
                normalized.push_str(pending);
                break;
            };

            normalized.push_str(&pending[..dont]);
            pending = &pending[dont..];
            ignoring = true;
        }
    }

    normalized
}

fn solve(input: &str) -> i32 {
    normalize(input)
        .split("mul(")
        .skip(1)
        .filter_map(|m| {
            let Some(closing) = m.find(")") else {
                return None;
            };

            let args = &m[0..closing];
            let Some((a, b)) = args.split_once(",") else {
                return None;
            };

            let Ok(a) = a.parse::<i32>() else {
                return None;
            };

            let Ok(b) = b.parse::<i32>() else {
                return None;
            };

            Some(a * b)
        })
        .sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "don't()mul(1,1)do()mul(1,1)do()don't()mul(1,1)";
        assert_eq!(solve(input), 1);
    }
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let total = sum_of_multiplications(&input);
    println!("Total sum of multiplications: {}", total);
    let total2 = solve(&input);
    println!("Total sum of multiplications: {}", total2);
    Ok(())
}