#![allow(unused_must_use)]
use itertools::Itertools;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Round {
    green: Option<u16>,
    blue: Option<u16>,
    red: Option<u16>,
}

const ROUND_SEPARATOR: &str = ";";

#[derive(Debug, Clone)]
pub struct Game(u16, Vec<Round>);

impl Display for Round {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "red: {:?} green: {:?} blue: {:?}",
            self.red, self.green, self.blue
        )
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Round {
            blue: None,
            green: None,
            red: None,
        };

        for color in s.split(",").map(|s| s.trim()).into_iter() {
            let (quantity, color) = color
                .splitn(2, " ")
                .collect_tuple::<(&str, &str)>()
                .unwrap_or(("", ""));

            let parsed_quantity = Some(quantity.parse::<u16>().unwrap_or(0));

            match color {
                "red" => round.red = parsed_quantity,
                "green" => round.green = parsed_quantity,
                "blue" => round.blue = parsed_quantity,
                _ => {}
            }
        }

        Ok(round)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, rounds) = s
            .trim()
            .splitn(2, ":")
            .collect_tuple::<(&str, &str)>()
            .unwrap_or(("", ""));

        let parsed_rounds = rounds
            .split(ROUND_SEPARATOR)
            .map(|g| g.parse().unwrap())
            .collect();

        Ok(Game(
            game_id.splitn(2, " ").collect::<Vec<&str>>()[1]
                .parse::<u16>()
                .unwrap_or(0),
            parsed_rounds,
        ))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir().expect("Couldn't find current dir");
    let input_path = PathBuf::from(current_dir).join("2023/day2/input.txt");

    let games_input = read_to_string(input_path)?;

    let parsed_games: Vec<Game> = games_input.lines().map(|g| g.parse().unwrap()).collect();

    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let sum_of_ids: u32 = parsed_games
        .clone()
        .into_iter()
        .fold(0, |acc, Game(id, rounds)| {
            for round in rounds.into_iter() {
                if round.red.unwrap_or(0) > total_red
                    || round.green.unwrap_or(0) > total_green
                    || round.blue.unwrap_or(0) > total_blue
                {
                    return acc;
                }
            }

            acc + id as u32
        });

    println!("{}", sum_of_ids);

    // part 2
    let sum_of_powers: u32 = parsed_games.into_iter().fold(0, |acc, Game(_, rounds)| {
        let min_red = rounds
            .clone()
            .into_iter()
            .fold(0, |acc, Round { red, .. }| {
                std::cmp::max(acc, red.unwrap_or(0))
            });

        let min_green = rounds
            .clone()
            .into_iter()
            .fold(0, |acc, Round { green, .. }| {
                std::cmp::max(acc, green.unwrap_or(0))
            });

        let min_blue = rounds.into_iter().fold(0, |acc, Round { blue, .. }| {
            std::cmp::max(acc, blue.unwrap_or(0))
        });

        acc + (min_blue as u32 * min_green as u32 * min_red as u32)
    });

    println!("{}", sum_of_powers);

    Ok(())
}
