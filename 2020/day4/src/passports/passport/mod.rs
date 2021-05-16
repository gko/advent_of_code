use regex::Regex;

mod display;
mod parse;
#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub enum Height {
    Centimeters(u8),
    Inches(u8),
}

#[derive(Debug, Clone)]
pub struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_code: Option<u8>,
}

impl Passport {
    // (Birth Year) - four digits; at least 1920 and at most 2002.
    fn is_birth_year_valid(&self) -> bool {
        match self.birth_year {
            None => false,
            Some(birth_year) => (1920..=2002).contains(&birth_year),
        }
    }

    // (Issue Year) - four digits; at least 2010 and at most 2020.
    fn is_issue_year_valid(&self) -> bool {
        match self.issue_year {
            None => false,
            Some(issue_year) => (2010..=2020).contains(&issue_year),
        }
    }

    // (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn is_expiration_year_valid(&self) -> bool {
        match self.expiration_year {
            None => false,
            Some(expiration_year) => (2020..=2030).contains(&expiration_year),
        }
    }

    // [> (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76. */
    fn is_height_valid(&self) -> bool {
        match self.height.clone() {
            None => false,
            Some(height) => match height {
                Height::Centimeters(height) => (150..=193).contains(&height),
                Height::Inches(height) => (59..=76).contains(&height),
            },
        }
    }

    // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn is_hair_color_valid(&self) -> bool {
        match &self.hair_color {
            None => false,
            Some(hair_color) => {
                let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

                re.is_match(&hair_color)
            }
        }
    }

    // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    fn is_eye_color_valid(&self) -> bool {
        match &self.eye_color {
            None => false,
            Some(eye_color) => matches!(
                eye_color.as_ref(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ),
        }
    }

    // (Passport ID) - a nine-digit number, including leading zeroes.
    fn is_passport_id_valid(&self) -> bool {
        match &self.passport_id {
            None => false,
            Some(passport_id) => {
                let re = Regex::new(r"^\d{9}$").unwrap();

                re.is_match(passport_id)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_birth_year_valid()
            || !self.is_issue_year_valid()
            || !self.is_expiration_year_valid()
            || !self.is_height_valid()
            || !self.is_hair_color_valid()
            || !self.is_eye_color_valid()
            || !self.is_passport_id_valid()
        {
            return false;
        }

        true
    }
}
