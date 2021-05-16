use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;

type Instruction<'a> = (&'a str, i16);

fn read_input() -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string("input.txt")?)
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

    println!("{:#?}", execute(&instructions));

    Ok(())
}
