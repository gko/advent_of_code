use itertools::Itertools;

fn find_invalid_number(numbers: &[u64], preamble: usize) -> Option<u64> {
    for i in preamble..(numbers.len() - 1) {
        let range = (i - preamble)..i;
        let prev_numbers = &numbers[range.clone()];

        let prev_indexes = (0..preamble as usize).combinations(2);
        if prev_indexes
            .into_iter()
            .all(|combo| numbers[i] != prev_numbers[combo[0]] + prev_numbers[combo[1]])
        {
            return Some(numbers[i]);
        }
    }

    None
}

fn sum_to_target(numbers: &[u64], target: u64, start_step: usize) -> Option<&[u64]> {
    let mut sum: &[u64];

    if start_step >= numbers.len() {
        return None;
    }

    for i in 0..&numbers.len() - start_step {
        sum = &numbers[i..(i + start_step)];

        if sum.iter().sum::<u64>() == target {
            return Some(sum);
        }
    }

    sum_to_target(numbers, target, start_step + 1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<u64> = input_data
        .lines()
        .map(|num| num.parse::<u64>().unwrap_or(0))
        .collect();

    let invalid_number = find_invalid_number(&numbers, 25).unwrap();

    println!("Part 1: {:#?}\n\n", invalid_number);

    let sum = sum_to_target(&numbers, invalid_number, 2).unwrap();
    println!(
        "Part 2: {:#?}",
        sum.iter().min().unwrap() + sum.iter().max().unwrap()
    );

    Ok(())
}
