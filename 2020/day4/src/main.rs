#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;
use std::fs::read_to_string;
use std::io;
mod passports;

use passports::*;

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input()?;
    let parsed_passports: Passports = input_data.parse().unwrap();
    let validated_passports: usize = parsed_passports.count_valid_passports();

    println!("valid passports: {}", validated_passports);

    Ok(())
}
