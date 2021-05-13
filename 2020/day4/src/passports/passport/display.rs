use std::fmt::{Display, Result, Formatter};
use std::collections::HashMap;

use super::{ Passport, PROPS };

impl Display for Passport {
    fn fmt(&self, f: &mut Formatter) -> Result {
        const DELIM: &str = "\n#=========================#\n";
        let props_desciption: HashMap<&str, &str> = [
            ("byr", "Birth Year"),
            ("iyr", "Issue Year"),
            ("eyr", "Expiration Year"),
            ("hgt", "Height"),
            ("hcl", "Hair Color"),
            ("ecl", "Eye Color"),
            ("pid", "Passport ID"),
            ("cid", "Country ID"),
        ]
        .iter()
        .cloned()
        .collect();

        let mut output = String::from(DELIM);
        output += "Passport:";

        for prop in &PROPS {
            if let Some(val) = self.0.get(&prop.to_string()) {
                output += "\n  ";
                output += props_desciption.get(prop).unwrap();
                output += ": ";
                output += val;
            }
        }

        output += "\nis valid: ";
        output += &self.is_valid().to_string();
        output += DELIM;

        write!(f, "{}", output)
    }
}
