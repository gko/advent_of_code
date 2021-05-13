use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

// wanted to do type Field = Vec<Pattern>
// but due to this couldn't:
// https://blog.rust-lang.org/2020/01/30/Rust-1.41.0.html#relaxed-restrictions-when-implementing-traits
#[derive(Debug)]
struct Field(Vec<Vec<Pattern>>);

#[derive(Debug)]
enum Pattern {
    Snow,
    Tree,
}

impl Field {
    fn count_trees(&self, step: (usize, usize)) -> u128 {
        let mut coords: (usize, usize) = (0, 0);
        let mut counter = 0;

        loop {
            if coords.1 >= self.0.len() - 1 {
                break counter;
            }

            coords.1 += step.1;
            coords.0 += step.0;

            let line = self.0.get(coords.1).unwrap();
            let mut pattern = line.get(coords.0);

            if let None = pattern {
                coords.0 -= line.len();
                pattern = line.get(coords.0)
            }

            if let Some(Pattern::Tree) = pattern {
                counter += 1
            }
        }
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = Field(vec![]);

        for line in s.lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut parsed_line = vec![];

            for ch in chars {
                parsed_line.push(match ch {
                    '.' => Pattern::Snow,
                    '#' => Pattern::Tree,
                    _ => panic!("Couldn't parse Field. Unexpected character: {}", ch),
                })
            }

            parsed.0.push(parsed_line)
        }

        Ok(parsed)
    }
}

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    let field: Field = input.parse().unwrap();

    println!(
        "{:#?}",
        field.count_trees((1, 1))
            * field.count_trees((3, 1))
            * field.count_trees((5, 1))
            * field.count_trees((7, 1))
            * field.count_trees((1, 2))
    );

    Ok(())
}
