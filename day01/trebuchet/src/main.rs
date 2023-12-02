//! AOC 2023 - Day 1
use std::{collections::HashMap, error::Error, fs};

static INTS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
static STRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Get the representative char of a number from a string
fn find_digit(number: &str) -> char {
    match number {
        "1" | "one" => '1',
        "2" | "two" => '2',
        "3" | "three" => '3',
        "4" | "four" => '4',
        "5" | "five" => '5',
        "6" | "six" => '6',
        "7" | "seven" => '7',
        "8" | "eight" => '8',
        "9" | "nine" => '9',
        _ => panic!("Unexpected input"),
    }
}

/// Find all 'numbers' within a string.
///
/// with_strs determines if to search for the word of the number, eg 'one'
fn find_numbers(line: &str, with_strs: bool) -> HashMap<usize, &str> {
    let mut numbers: HashMap<usize, &str> = HashMap::new();
    let mut eval = vec![&INTS];
    if with_strs {
        eval.push(&STRS)
    }

    for e in eval {
        for num in e {
            for m in line.match_indices(num).collect::<Vec<_>>() {
                numbers.insert(m.0, m.1);
            }
        }
    }
    numbers
}

/// Run the challenge
fn run(data: &str, with_strs: bool) -> Result<u32, Box<dyn Error>> {
    let mut numbers: Vec<u32> = Vec::new();
    for line in data.split("\n").into_iter().filter(|l| l != &"") {
        let digits = find_numbers(line, with_strs);
        numbers.push(
            format!(
                "{}{}",
                find_digit(digits.get(digits.keys().min().expect("")).expect("")),
                find_digit(digits.get(digits.keys().max().expect("")).expect(""))
            )
            .parse()?,
        );
    }
    Ok(numbers.iter().fold(0, |acc, x| acc + x))
}

/// Entrypoint
fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data.txt")?;
    println!("Part 1: {}", run(&data, false)?);
    println!("Part 2: {}", run(&data, true)?);
    Ok(())
}
