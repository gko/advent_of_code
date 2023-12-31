use std::env::current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir().expect("could not find current dir");
    let input = read_to_string(PathBuf::from(current_dir).join("2023/day1/input.txt"))?;
    let sums: Vec<u32> = input
        .lines()
        .map(|line| {
            let numeric_chars: Vec<char> = line.chars().filter(|ch| ch.is_numeric()).collect();
            format!(
                "{}{}",
                numeric_chars[0],
                numeric_chars[numeric_chars.len() - 1]
            )
            .parse::<u32>()
            .unwrap_or(0)
        })
        .collect();

    println!("{:?}", sums.iter().sum::<u32>());

    Ok(())
}
