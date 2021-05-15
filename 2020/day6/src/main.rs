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

    println!(
        "Part 1:\n {:#?}",
        groups.iter().fold(0, |acc, s| acc + s.len())
    );

    let groups = input_data
        .split("\n\n")
        .map(|s| {
            let mut questions: HashSet<char> = HashSet::new();
            let all_groups_answers = s.trim().replace('\n', "");

            for ch in all_groups_answers.chars() {
                questions.insert(ch);
            }

            let group_length = s.trim().split('\n').count();

            questions
                .iter()
                .filter(|q| {
                    all_groups_answers.chars().filter(|ch| ch == *q).count() == group_length
                })
                .count()
        })
        .collect::<Vec<usize>>();

    println!("\nPart 2:\n {:#?}", groups.iter().sum::<usize>());

    Ok(())
}
