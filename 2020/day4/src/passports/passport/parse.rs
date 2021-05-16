use itertools::Itertools;
use std::str::FromStr;

use super::{Height, Passport};

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_code: None,
        };
        let props: Vec<String> = s.split_whitespace().map(String::from).collect();

        for prop in props {
            let (name, val) = prop.splitn(2, ':').collect_tuple().unwrap_or(("", ""));

            match name {
                "byr" => passport.birth_year = val.parse::<u16>().ok(),
                "iyr" => passport.issue_year = val.parse::<u16>().ok(),
                "eyr" => passport.expiration_year = val.parse::<u16>().ok(),
                "hgt" => {
                    let height = val[..val.len()-2].parse::<u8>().unwrap_or(0);

                    passport.height = Some(if val.ends_with("cm") {
                        Height::Centimeters(height)
                    } else {
                        Height::Inches(height)
                    })
                }
                "hcl" => passport.hair_color = Some(val.to_string()),
                "ecl" => passport.eye_color = Some(val.to_string()),
                "pid" => passport.passport_id = Some(val.to_string()),
                "cid" => passport.country_code = val.parse::<u8>().ok(),
                _ => (),
            };
        }

        Ok(passport)
    }
}
