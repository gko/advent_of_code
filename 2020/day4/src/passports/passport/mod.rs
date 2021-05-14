use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

mod display;
mod deref;
mod parse;
#[cfg(test)]
mod test;

const NECESSARY_PROPS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[derive(Debug, Clone)]
pub enum PassportProp {
    BirthYear(Option<u16>),
    IssueYear(Option<u16>),
    ExpirationYear(Option<u16>),
    Height(Option<String>),
    HairColor(Option<String>),
    EyeColor(Option<String>),
    PassportID(Option<String>),
    CountryCode(Option<u8>),
}

#[derive(Debug, Clone)]
pub struct Passport(HashMap<String, PassportProp>);

impl Passport {
    fn is_prop_valid(prop: PassportProp) -> bool {
        match prop {
            // (Birth Year) - four digits; at least 1920 and at most 2002.
            PassportProp::BirthYear(prop) => {
                let byr = prop.unwrap_or_else(|| 0);
                RangeInclusive::from(1920..=2002).contains(&byr)
            }
            // (Issue Year) - four digits; at least 2010 and at most 2020.
            PassportProp::IssueYear(prop) => {
                let byr = prop.unwrap_or_else(|| 0);
                RangeInclusive::from(2010..=2020).contains(&byr)
            }
            // (Expiration Year) - four digits; at least 2020 and at most 2030.
            PassportProp::ExpirationYear(prop) => {
                let byr = prop.unwrap_or_else(|| 0);
                RangeInclusive::from(2020..=2030).contains(&byr)
            }
            // [> (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76. */
            PassportProp::Height(prop) => {
                let hgt = prop.unwrap_or_else(|| String::from(""));
                let re = Regex::new(r"(?P<height>\d{2,3})(?P<measure>[a-z]{2})").unwrap();

                match re.captures(&hgt[..]) {
                    Some(capture) => {
                        let height = &capture["height"].parse::<u16>().unwrap_or_else(|_| 0);
                        let measure = &capture["measure"];

                        match measure {
                            "cm" => RangeInclusive::from(150..=193).contains(height),
                            "in" => RangeInclusive::from(59..=76).contains(height),
                            _ => false,
                        }
                    }
                    _ => return false,
                }
            }
            // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            PassportProp::HairColor(prop) => {
                let hcl = prop.unwrap_or_else(|| String::from(""));
                let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

                re.is_match(&hcl)
            }
            // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            PassportProp::EyeColor(prop) => {
                let ecl = prop.unwrap_or_else(|| String::from(""));

                match ecl.as_ref() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            }
            // (Passport ID) - a nine-digit number, including leading zeroes.
            PassportProp::PassportID(prop) => {
                let re = Regex::new(r"^\d{9}$").unwrap();

                re.is_match(&prop.unwrap_or_else(|| String::from("")))
            }
            _ => false,
        }
    }

    pub fn is_valid(&self) -> bool {
        let derefed = &*self;

        for prop in NECESSARY_PROPS.iter() {
            let val = derefed.get(&prop.to_string());

            match val {
                None => return false,
                Some(val) => {
                    if !Passport::is_prop_valid(val.clone()) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
