mod grid;
mod schema;
mod schema_row;

use std::env::current_dir;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

use grid::Grid;

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir().expect("Could not find cwd");
    let input_path = PathBuf::from(current_dir).join("2023/day3/input.txt");
    let input: Grid = read_to_string(input_path)?.parse().unwrap();

    // part 1
    println!(
        "{:?}",
        input
            .numbers_with_adjacent_symbols()
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .unwrap_or_default()
    );

    // part 2
    println!(
        "{:?}",
        input
            .get_gears()
            .iter()
            .copied()
            .reduce(|a, b| a + b)
            .unwrap_or_default()
    );

    Ok(())
}
