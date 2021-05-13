use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

mod deref;
mod display;
mod parse;
#[cfg(test)]
mod test;

pub const PROPS: [&str; 7] = [
    "byr", // Birth Year
    "iyr", // Issue Year
    "eyr", // Expiration Year
    "hgt", // Height
    "hcl", // Hair Color
    "ecl", // Eye Color
    "pid", // Passport ID
];

#[derive(Debug, Clone)]
pub struct Passport(HashMap<String, String>);

impl Passport {
    fn is_prop_valid(prop: &str, val: &str) -> bool {
        let is_between = |num: &str, range: RangeInclusive<i32>| -> bool {
            let parsed_value = num.trim().parse::<i32>().unwrap_or_else(|err| {
                println!("{:#?} : {}", err, num);
                0
            });

            range.contains(&parsed_value)
        };

        return match prop {
            // (Birth Year) - four digits; at least 1920 and at most 2002.
            "byr" => is_between(val, 1920..=2002),
            // (Issue Year) - four digits; at least 2010 and at most 2020.
            "iyr" => is_between(val, 2010..=2020),
            // (Expiration Year) - four digits; at least 2020 and at most 2030.
            "eyr" => is_between(val, 2020..=2030),
            // [> (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76. */
            "hgt" => {
                let re = Regex::new(r"(?P<height>\d{2,3})(?P<measure>[a-z]{2})").unwrap();

                match re.captures(val) {
                    Some(capture) => {
                        let height = &capture["height"];
                        let measure = &capture["measure"];

                        if measure == "cm" {
                            return is_between(height, 150..=193);
                        } else if measure == "in" {
                            return is_between(height, 59..=76);
                        }

                        return false;
                    }
                    _ => return false,
                }
            }
            // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            "hcl" => {
                let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

                re.is_match(val)
            }
            // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            "ecl" => match val {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            // (Passport ID) - a nine-digit number, including leading zeroes.
            "pid" => {
                let re = Regex::new(r"^\d{9}$").unwrap();

                re.is_match(val)
            }
            _ => false,
        };
    }

    pub fn is_valid(&self) -> bool {
        let derefed = &*self;

        for prop in &PROPS {
            let val = derefed.get(&prop.to_string());

            match val {
                None => return false,
                Some(val) => {
                    if !Passport::is_prop_valid(prop, val) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
