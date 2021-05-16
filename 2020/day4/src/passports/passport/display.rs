use std::fmt::{Display, Formatter, Result};

use super::Passport;

impl Display for Passport {
    fn fmt(&self, f: &mut Formatter) -> Result {
        const DELIM: &str = "\n#=========================#\n";

        write!(f, "{}Passport:", DELIM);

        if let Some(birth_year) = self.birth_year {
            write!(f, "\nBirth Year: {:?}", birth_year);
        }

        if let Some(issue_year) = self.issue_year {
            write!(f, "\nIssue Year: {:?}", issue_year);
        }

        if let Some(expiration_year) = self.expiration_year {
            write!(f, "\nExpiration Year: {:?}", expiration_year);
        }

        if let Some(height) = self.height.clone() {
            write!(f, "\nHeight: {:#?}", height);
        }

        if let Some(hair_color) = self.hair_color.clone() {
            write!(f, "\nHair Color: {:?}", hair_color);
        }

        if let Some(eye_color) = self.eye_color.clone() {
            write!(f, "\nEye Color: {:?}", eye_color);
        }

        if let Some(passport_id) = self.passport_id.clone() {
            write!(f, "\nPassport ID: {:?}", passport_id);
        }

        if let Some(country_code) = self.country_code {
            write!(f, "\nCountry Code: {:?}", country_code,);
        }

        write!(
            f,
            "\n\nis valid: {}{}",
            &self.is_valid().to_string(),
            DELIM
        )
    }
}
