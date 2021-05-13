use std::ops::Deref;
use std::collections::HashMap;

use super::Passport;

impl Deref for Passport {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
