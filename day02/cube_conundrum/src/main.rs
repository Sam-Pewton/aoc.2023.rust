//! AOC 2023 - Day 2
use std::{error::Error, fs};

/// Retrieve the 'minimum set' required for a game and check if the game is possible for
/// a given number of red, green, and blue cubes.
fn min_set_is_possible(
    round: &Vec<Vec<&str>>,
    r: usize,
    g: usize,
    b: usize,
) -> Result<((usize, usize, usize), bool), Box<dyn Error>> {
    let mut possible = true;
    let mut set = (0, 0, 0);
    for n in round {
        for c in n {
            let temp: Vec<&str> = c.split(" ").collect();
            let count: usize = temp[0].parse()?;
            match temp[1] {
                "red" => {
                    if count > r {
                        possible = false;
                    };
                    if count > set.0 {
                        set.0 = count;
                    };
                }
                "green" => {
                    if count > g {
                        possible = false;
                    };
                    if count > set.1 {
                        set.1 = count;
                    };
                }
                "blue" => {
                    if count > b {
                        possible = false;
                    };
                    if count > set.2 {
                        set.2 = count;
                    };
                }
                _ => (),
            }
        }
    }
    Ok((set, possible))
}

/// Run the challenge
///
/// Runs the 'min_set_is_possible' on each of the games. Otherwise, most is string parsing.
fn run(data: &str, r: usize, g: usize, b: usize) -> Result<(), Box<dyn Error>> {
    let mut possible_games = Vec::new();
    let mut minimum_sets = Vec::new();
    for game in data.split("\n").filter(|l| l != &"") {
        let s = game.split(": ").collect::<Vec<&str>>();
        let disp: Vec<Vec<&str>> = s[1].split("; ").map(|x| x.split(", ").collect()).collect();
        let res = min_set_is_possible(&disp, r, g, b).expect("");
        minimum_sets.push(res.0);
        if res.1 {
            possible_games.push(s[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>()?);
        }
    }
    println!(
        "Part 1: {:?}",
        possible_games.iter().fold(0, |acc, x| x + acc)
    );
    println!(
        "Part 2: {:?}",
        minimum_sets
            .iter()
            .map(|x| x.0 * x.1 * x.2)
            .fold(0, |acc, x| x + acc)
    );
    Ok(())
}

/// Entrypoint
fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("data.txt")?;
    run(&data, 12, 13, 14)?;
    Ok(())
}
