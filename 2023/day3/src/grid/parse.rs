use std::str::FromStr;

use super::Grid;
use crate::schema_row::SchemaRow;
use crate::schema::Schema;

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<SchemaRow> = s
            .lines()
            .map(|row| {
                let mut num: Vec<char> = vec![];
                let mut empty_len: usize = 1;

                SchemaRow(
                    row.chars()
                        .enumerate()
                        .filter_map(|(idx, ch)| match ch {
                            '.' => {
                                let next_char = row.chars().nth(idx + 1).unwrap_or_default();
                                if next_char == '.' {
                                    empty_len += 1;
                                    None
                                } else {
                                    let empty_space = Schema::Empty(empty_len);
                                    empty_len = 1;
                                    Some(empty_space)
                                }
                            }
                            _ if ch.is_numeric() => {
                                num.push(ch);
                                let next_char = row.chars().nth(idx + 1).unwrap_or_default();
                                if next_char.is_numeric() {
                                    None
                                } else {
                                    let parsed_number =
                                        num.iter().collect::<String>().parse::<u32>().unwrap_or(0);
                                    let length = num.len();
                                    num = vec![];
                                    Some(Schema::Number(parsed_number, idx - (length - 1), length))
                                }
                            }
                            '*' => Some(Schema::Gear),
                            _ => Some(Schema::Symbol(ch)),
                        })
                        .collect::<Vec<Schema>>(),
                )
            })
            .collect::<Vec<SchemaRow>>();

        Ok(Grid(grid))
    }
}