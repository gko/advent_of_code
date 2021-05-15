use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::str::FromStr;

fn read_input() -> Result<String, io::Error> {
    let result = read_to_string("input.txt")?;

    Ok(result)
}

#[derive(Debug, Clone, Hash, Eq)]
struct Bag {
    name: String,
    contains: Option<Vec<(u16, Bag)>>,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

type Bags = Vec<Bag>;

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" bags contain ").collect::<Vec<&str>>();

        if let [name, contains_str] = &split[..] {
            let mut contains: Vec<(u16, Bag)> = Vec::new();

            for bag in contains_str.split(',') {
                let re =
                    Regex::new(r"(?P<can_contain>\d{1,})\s(?P<name>[a-z\s]*)\sbags?.?").unwrap();

                let captured = re.captures(bag);

                match captured {
                    Some(capture) => {
                        let count = u16::from_str(&capture["can_contain"]).unwrap_or(0);
                        let name = &capture["name"];

                        contains.push((
                            count,
                            Bag {
                                name: name.to_string(),
                                contains: None,
                            },
                        ))
                    }
                    _ => (),
                }
            }

            return Ok(Bag {
                name: name.to_string(),
                contains: Some(contains),
            });
        }

        panic!("no bag")
    }
}

fn count_bags(bag_name: &str, bags: &Bags, except: &mut HashSet<Bag>) -> u16 {
    let bags_count: Vec<Option<u16>> = bags
        .iter()
        .map(|bag| match &bag.contains {
            Some(vec) => {
                if let Some(_) = except.iter().find(|except_bag| except_bag.name == bag.name) {
                    return None;
                }

                let found = vec
                    .iter()
                    .find(|(_, can_contain_bag)| can_contain_bag.name == *bag_name);

                match found {
                    Some((_, _)) => {
                        except.insert(bag.clone());

                        Some(1 + count_bags(&bag.name, bags, except))
                    }
                    _ => None,
                }
            }
            _ => None,
        })
        .collect();

    bags_count.iter().fold(0, |acc, count| match count {
        None => acc,
        Some(count) => acc + count,
    })
}

fn count_containing(bag_name: &str, bags: &Bags, initial_name: &str) -> u32 {
    let bag = bags.iter().find(|b| b.name == bag_name).unwrap();

    if let Some(contains) = bag.clone().contains {
        contains.iter().fold(0, |acc, (count, _bag)| {
            let _count = count_containing(&_bag.name, bags, initial_name);

            acc + u32::from(*count) + (u32::from(*count) * _count)
        })
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = read_input().unwrap();

    let bags: Vec<Bag> = input_data
        .lines()
        .map(|bag_description| Bag::from_str(bag_description).unwrap())
        .collect();

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
