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
            line.chars()
                .filter(|ch| ch.is_numeric())
                .collect::<Vec<char>>()
        })
        .map(|chars| {
            let mut num = chars[0].to_string();
            num.push(chars[chars.len() - 1]);
            num.parse::<u32>().unwrap_or(0)
        })
        .collect(); //.map::u16(|num| num.parse::u16().unwrap_or(0))

    println!("{:?}", sums.iter().sum::<u32>());

    Ok(())
}
