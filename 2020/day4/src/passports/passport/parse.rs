use itertools::Itertools;
use std::str::FromStr;
use std::collections::HashMap;

use super::Passport;

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed = Passport(HashMap::new());
        let props: Vec<&str> = s.split_whitespace().collect();

        for prop in props {
            let (name, val) = prop.splitn(2, ':').collect_tuple().unwrap();
            parsed.0.insert(String::from(name), String::from(val));
        }

        Ok(parsed)
    }
}
