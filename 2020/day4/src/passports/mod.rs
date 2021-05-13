mod parse;

pub mod passport;

use passport::*;

#[derive(Debug, Clone)]
pub struct Passports(pub Vec<Passport>);

impl Passports {
    pub fn count_valid_passports(&self) -> usize {
        let valid = &self
            .0
            .clone()
            .into_iter()
            .filter(|pass| {
                if pass.is_valid() {
                    println!("{}", pass);
                }

                pass.is_valid()
            })
            .collect::<Vec<Passport>>()
            .len();

        *valid
    }
}
