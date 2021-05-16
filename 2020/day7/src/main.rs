use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::ops::Deref;
use std::str::FromStr;

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;

    Ok(result)
}

struct Bags(HashMap<String, Vec<(u8, String)>>);

impl Deref for Bags {
    type Target = HashMap<String, Vec<(u8, String)>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Bags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = HashMap::new();

        for bag in s.lines() {
            let split = bag.split(" bags contain ").collect::<Vec<&str>>();

            if let [name, contains_str] = &split[..] {
                let mut contains: Vec<(u8, String)> = Vec::new();

                for bag in contains_str.split(',') {
                    let re = Regex::new(r"(?P<can_contain>\d{1,})\s(?P<name>[a-z\s]*)\sbags?.?")
                        .unwrap();

                    let captured = re.captures(bag);

                    if let Some(capture) = captured {
                        let count = u8::from_str(&capture["can_contain"]).unwrap_or(0);
                        let name = &capture["name"];

                        contains.push((count, name.to_string()))
                    }
                }

                parsed.insert(name.to_string(), contains);
            }
        }

        Ok(Bags(parsed))
    }
}

fn count_bags(bag_name: &str, bags: &Bags, except: &mut HashSet<String>) -> u16 {
    let bags_count: Vec<Option<u16>> = bags.keys().map(|_bag_name| {
        if except.contains(_bag_name) {
            return None;
        }

        let found = bags[_bag_name]
            .iter()
            .find(|(_, can_contain_bag)| can_contain_bag == bag_name);

        match found {
            Some((_, _)) => {
                except.insert(_bag_name.clone());

                Some(1 + count_bags(_bag_name.as_ref(), bags, except))
            }
            _ => None,
        }
    }).collect();

    bags_count.iter().fold(0, |acc, count| match count {
        None => acc,
        Some(count) => acc + count,
    })
}

fn count_containing(bag_name: &str, bags: &Bags, initial_name: &str) -> u32 {
    bags[bag_name].iter().fold(0, |acc, (count, _bag_name)| {
        let _count = count_containing(&_bag_name, bags, initial_name);

        acc + u32::from(*count) + (u32::from(*count) * _count)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input().unwrap();

    let bags = Bags::from_str(&input_data).unwrap();

    println!(
        "Part 1:\n {:#?}",
        count_bags("shiny gold", &bags, &mut HashSet::new())
    );

    println!(
        "\nPart 2:\n {:#?}",
        count_containing("shiny gold", &bags, "shiny gold")
    );

    Ok(())
}
