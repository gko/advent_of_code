use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;

type Instruction<'a> = (&'a str, i16);

fn read_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("input.txt")
}

fn execute(instructions: &[Instruction]) -> i32 {
    let mut current_pos: usize = 0;
    let mut history: HashSet<usize> = HashSet::new();
    let mut result: i32 = 0;

    loop {
        if current_pos >= instructions.len() || history.contains(&current_pos) {
            break;
        }

        history.insert(current_pos);

        let (command, count) = instructions[current_pos];

        match command {
            "acc" => {
                result += i32::from(count);
                current_pos += 1
            }
            "jmp" => {
                if count < 0 {
                    let abs_c = count.abs() as usize;

                    if abs_c <= current_pos {
                        current_pos -= abs_c
                    } else {
                        break;
                    }
                } else {
                    current_pos += count as usize
                }
            }
            _ => current_pos += 1,
        }
    }

    result
}

fn execute_with_swap(
    instructions: &[Instruction],
    swap_position: &usize,
) -> Result<i32, &'static str> {
    let mut current_pos: usize = 0;
    let mut history: HashSet<usize> = HashSet::new();
    let mut result: i32 = 0;

    loop {
        if current_pos >= instructions.len() {
            return Ok(result);
        }

        if history.contains(&current_pos) {
            return Err("Infinite loop");
        }

        history.insert(current_pos);

        let (mut command, count) = instructions[current_pos];

        if current_pos == *swap_position && ["nop", "jmp"].contains(&command) {
            command = if command == "nop" { "jmp" } else { "nop" };
        }

        match command {
            "acc" => {
                result += i32::from(count);
                current_pos += 1
            }
            "jmp" => {
                if count < 0 {
                    let abs_c = count.abs() as usize;

                    if abs_c <= current_pos {
                        current_pos -= abs_c
                    } else {
                        return Err("Out of bounds");
                    }
                } else {
                    current_pos += count as usize
                }
            }
            _ => current_pos += 1,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input().unwrap();

    let instructions: Vec<Instruction> = input_data
        .lines()
        .map(|line| {
            let (command, count) = line
                .trim()
                .splitn(2, ' ')
                .collect_tuple()
                .unwrap_or(("nop", ""));

            (command, count.parse::<i16>().unwrap_or(0))
        })
        .collect();

    println!("Part 1: {:#?}\n\n", execute(&instructions));

    println!(
        "Part 2: {:#?}",
        instructions
            .iter()
            .enumerate()
            .find(|(i, _)| match execute_with_swap(&instructions, &i) {
                Ok(res) => {
                    println!("Result: {}", res);
                    true
                }
                Err(_) => false,
            })
    );

    Ok(())
}
