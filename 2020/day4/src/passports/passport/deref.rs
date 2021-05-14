use std::collections::HashMap;
use std::ops::Deref;

use super::{Passport, PassportProp};

impl Deref for Passport {
    type Target = HashMap<String, PassportProp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
