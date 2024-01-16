mod parse;

use std::ops::Index;
use std::vec::Vec;

use crate::schema::Schema;
use crate::schema_row::SchemaRow;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Grid(Vec<SchemaRow>);

impl Index<usize> for Grid {
    type Output = SchemaRow;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Grid {
    fn area_around_schema_item(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut indices_to_check = Vec::new();
        let item_length = &self[r][c].len();

        if c > 0 {
            indices_to_check.push((r, c - 1));

            if r > 0 {
                indices_to_check.push((r - 1, c - 1));
            }

            if r < self.0.len() - 1 {
                indices_to_check.push((r + 1, c - 1));
            }
        }

        if c + item_length < self.0[r].len() - 1 {
            indices_to_check.push((r, c + item_length));

            if r > 0 {
                indices_to_check.push((r - 1, c + item_length));
            }

            if r < self.0.len() - 1 {
                indices_to_check.push((r + 1, c + item_length));
            }
        }

        for i in c..=c + item_length {
            if r > 0 {
                indices_to_check.push((r - 1, i));
            }

            if r < &self.0.len() - 1 {
                indices_to_check.push((r + 1, i));
            }
        }

        indices_to_check
    }

    fn has_adjacent_item<F>(&self, row: usize, col: usize, criteria: F) -> bool
    where
        F: Fn(&Schema) -> bool,
    {
        let indices_to_check = &self.area_around_schema_item(row, col);

        for &(r, c) in indices_to_check {
            if criteria(&self[r][c]) == true {
                return true;
            }
        }

        false
    }

    fn all_adjacent_items(&self, row: usize, col: usize) -> Vec<&Schema> {
        let mut adjacent_items = HashSet::new();
        let indices_to_check = &self.area_around_schema_item(row, col);

        for &(r, c) in indices_to_check {
            adjacent_items.insert(&self[r][c]);
        }

        adjacent_items.into_iter().collect()
    }

    pub fn numbers_with_adjacent_symbols(&self) -> Vec<u32> {
        let filter_numbers = |(row_idx, row): (usize, &SchemaRow)| {
            let a = row
                .0
                .iter()
                .filter_map(|item| -> Option<u32> {
                    if let Schema::Number(num, col, _) = item {
                        if self.has_adjacent_item(row_idx, *col, |item| {
                            matches!(item, Schema::Symbol(_) | Schema::Gear)
                        }) {
                            Some(num.to_owned())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>();

            a
        };

        self.0
            .iter()
            .enumerate()
            .flat_map(filter_numbers)
            .collect::<Vec<u32>>()
    }

    pub fn get_gears(&self) -> Vec<u32> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, row)| -> Vec<u32> {
                let mut a = vec![];

                for i in 0..=row.len() {
                    if let Schema::Gear = &row[i] {
                        let all_adjacent_items = self.all_adjacent_items(idx, i);
                        let adjacent_number_items = all_adjacent_items
                            .iter()
                            .filter_map(|&item| match item {
                                Schema::Number(num, _, _) => Some(num.to_owned()),
                                _ => None,
                            })
                            .collect::<Vec<u32>>();

                        if adjacent_number_items.len() == 2 {
                            a.push(
                                adjacent_number_items
                                    .iter()
                                    .copied()
                                    .reduce(|a, b| a * b)
                                    .unwrap_or_default(),
                            )
                        }
                    }
                }

                a
            })
            .flatten()
            .collect::<Vec<u32>>()
    }
}
