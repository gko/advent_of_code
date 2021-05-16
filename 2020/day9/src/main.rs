use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_data = std::fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<u64> = input_data
        .lines()
        .map(|num| num.parse::<u64>().unwrap_or(0))
        .collect();

    let preamble = 25;

    for i in preamble..numbers.len() - 1 {
        let range = (i - preamble)..i;
        let prev_numbers = &numbers[range.clone()];
        // println!("{:#?} {} {:#?}", range, numbers[i], prev_numbers);

        let prev_indexes = (0..preamble).combinations(2);
        if prev_indexes.into_iter().all(|combo| {
            // println!("{:#?}", combo);
            numbers[i] != prev_numbers[combo[0]] + prev_numbers[combo[1]]
        }) {
            // println!("------------- {}", numbers[i]);
            break;
        }
    }

    Ok(())
}
