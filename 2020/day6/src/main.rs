use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::io;

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input().unwrap();
    let groups = input_data
        .split("\n\n")
        .map(|s| {
            let mut uniq: HashSet<char> = HashSet::new();
            for ch in s.replace('\n', "").chars() {
                uniq.insert(ch);
            }

            uniq.iter().collect::<String>()
        })
        .collect::<Vec<String>>();

    println!("Part 1:\n {:#?}", groups.iter().fold(0, |acc, s| acc + s.len()));

    Ok(())
}
