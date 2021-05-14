use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

use super::{Passport, PassportProp};

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport_data = HashMap::new();
        let props: Vec<String> = s.split_whitespace().map(String::from).collect();

        for prop in props {
            let (name, val) = prop.splitn(2, ':').collect_tuple().unwrap();

            match name {
                "byr" => passport_data.insert(
                    String::from(name),
                    PassportProp::BirthYear(val.parse::<u16>().ok()),
                ),
                "iyr" => passport_data.insert(
                    String::from(name),
                    PassportProp::IssueYear(val.parse::<u16>().ok()),
                ),
                "eyr" => passport_data.insert(
                    String::from(name),
                    PassportProp::ExpirationYear(val.parse::<u16>().ok()),
                ),
                "hgt" => passport_data.insert(
                    String::from(name),
                    PassportProp::Height(Some(val.to_string())),
                ),
                "hcl" => passport_data.insert(
                    String::from(name),
                    PassportProp::HairColor(Some(val.to_string())),
                ),
                "ecl" => passport_data.insert(
                    String::from(name),
                    PassportProp::EyeColor(Some(val.to_string())),
                ),
                "pid" => passport_data.insert(
                    String::from(name),
                    PassportProp::PassportID(Some(val.to_string())),
                ),
                "cid" => passport_data.insert(
                    String::from(name),
                    PassportProp::CountryCode(val.parse::<u8>().ok()),
                ),
                _ => None,
            };
        }

        Ok(Passport(passport_data))
    }
}
