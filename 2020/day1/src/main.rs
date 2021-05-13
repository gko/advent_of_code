use itertools::Itertools;
use std::fs::read_to_string;

fn read_input(filename: String) -> Vec<i32> {
    let result = read_to_string(filename).expect("error reading file");

    result
        .split_whitespace()
        .map(|l| l.parse::<i32>().unwrap_or_default())
        .collect()
}

fn main() {
    let vec: Vec<i32> = read_input(String::from("input.txt"));

    let solve = || -> Result<i32, String> {
        // to solve part 2 just change to 3
        for c in vec.iter().combinations(2) {
            let sum: i32 = c.clone().into_iter().sum();
            if sum == 2020 {
                return Ok(c.into_iter().fold(1, |mul, v| v * mul));
            }
        }

        Err(String::from("failed to find a combo"))
    };

    println!("from block: {:?}", solve().unwrap());
}
