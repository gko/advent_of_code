use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use super::Passport;

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

        write!(
            f,
            "{}\nPassport:",
            DELIM
        );

        for prop in self.0.keys() {
            if let Some(val) = self.0.get(&prop.to_string()) {
                write!(
                    f,
                    "\n  {:?}: {:?}",
                    props_desciption.get(&prop.as_str()).unwrap(),
                    val
                );
            }
        }

        write!(
            f,
            "\n  is valid: {}\n{}",
            &self.is_valid().to_string(),
            DELIM
        )
    }
}
