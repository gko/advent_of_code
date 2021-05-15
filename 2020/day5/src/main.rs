use std::error::Error;
use std::fs::read_to_string;
use std::io;

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;

    Ok(result)
}

type Place = (u16, u16);

fn find_place(instructions: &str) -> Place {
    let place_instructions = instructions.trim()[(instructions.len() - 3)..].chars();
    let row_instructions = instructions.trim()[..(instructions.len() - 3)].chars();

    let mut step = 128;
    let mut rows = 1..step;
    let mut row: u16 = 0;

    for letter in row_instructions {
        step /= 2;
        row = match letter {
            'F' => {
                if rows.len() == 1 {
                    rows.start
                } else {
                    rows = rows.start..(rows.end - step);
                    0
                }
            }
            _ => {
                if rows.len() == 1 {
                    rows.end
                } else {
                    rows = (rows.start + step)..rows.end;
                    0
                }
            }
        };
    }

    step = 8;
    rows = 1..step;
    let mut place: u16 = 0;

    for letter in place_instructions {
        step /= 2;
        place = match letter {
            'L' => {
                if rows.len() == 1 {
                    rows.start
                } else {
                    rows = rows.start..(rows.end - step);
                    0
                }
            }
            _ => {
                if rows.len() == 1 {
                    rows.end
                } else {
                    rows = (rows.start + step)..rows.end;
                    0
                }
            }
        };
    }

    (row - 1, place - 1)
}

fn place_id((row, place): Place) -> u32 {
    u32::from(row) * 8 + u32::from(place)
}

fn find_missing_places(places: Vec<Place>) -> Option<Place> {
    let mut all_available_places: Vec<Place> = Vec::new();

    for i in 0..127 {
        for j in 0..7 {
            all_available_places.push((i, j))
        }
    }

    let all_missing_places: Vec<Place> = all_available_places
        .iter()
        .filter(|available| {
            let found = places
                .iter()
                .find(|booked| booked.0 == available.0 && booked.1 == available.1);

            found.is_none()
        })
        .cloned()
        .collect();

    all_missing_places
        .iter()
        .find(|missing_place| {
            if missing_place.0 == 0 || missing_place.1 == 0 {
                return false
            }

            let missing_place_id = place_id(**missing_place);

            let found = all_missing_places.iter().find(|place| {
                let _place_id = place_id(**place);

                _place_id == missing_place_id + 1
                    || _place_id == missing_place_id - 1
            });

            found.is_none()
        })
        .cloned()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input().unwrap();

    let ids: Vec<u32> = input_data
        .lines()
        .map(|instruction| place_id(find_place(instruction)))
        .collect();

    println!("Part 1:\n  {:#?}", ids.iter().max());

    let places: Vec<Place> = input_data
        .lines()
        .map(|instruction| find_place(instruction))
        .collect();

    println!("\nPart 2:\n  {:#?}", find_missing_places(places));

    Ok(())
}
