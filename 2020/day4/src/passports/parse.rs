use std::str::FromStr;

use super::Passports;

const SEPARATOR: &str = "\n\n";

impl FromStr for Passports {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = String::from(s)
            .split(SEPARATOR)
            .map(|passport_str| passport_str.parse().unwrap())
            .collect();

        Ok(Passports(parsed))
    }
}
