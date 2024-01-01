use std::env::current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::vec::Vec;

fn replace_nums(s: &str) -> u32 {
    let nums = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut num_pos: Vec<(usize, String)> = vec![];

    for (idx, ch) in s.chars().enumerate() {
        if ch.is_numeric() {
            num_pos.push((idx, ch.to_string()))
        }
    }

    for (digit, num_str) in nums.iter().enumerate() {
        if let Some(found_idx) = s.rfind(num_str) {
            num_pos.push((found_idx, digit.to_string()))
        }
    }

    num_pos.sort_by(|(a, _), (b, _)| a.cmp(b));

    let default_value = (0, String::new());
    let first = num_pos.first().unwrap_or(&default_value);
    let last = num_pos.last().unwrap_or(&default_value);

    let mut combined: String = first.1.clone();
    combined.push_str(&last.1);
    combined.parse::<u32>().unwrap_or(0)
}

fn calibration_numbers(input: Vec<&str>) -> Vec<u32> {
    input
        .iter()
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
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir().expect("could not find current dir");
    let input = read_to_string(PathBuf::from(current_dir).join("2023/day1/input.txt"))?;
    let sums: Vec<u32> = calibration_numbers(input.lines().collect());

    println!("{:?}", sums.iter().sum::<u32>());

    // part 2
    let _sums: Vec<u32> = input.lines().map(replace_nums).collect();

    println!("{:?}", _sums.iter().sum::<u32>());

    Ok(())
}
