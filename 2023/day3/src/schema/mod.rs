use std::hash::*;

#[derive(Debug, Clone)]
pub enum Schema {
    Empty(usize),
    Symbol(char),
    Number(u32, usize, usize),
    Gear,
}

impl Schema {
    pub fn len(&self) -> usize {
        match &self {
            &Self::Empty(l) => *l,
            &Self::Symbol(_) => 1,
            &Self::Number(_, _, l) => *l,
            &Self::Gear => 1,
        }
    }
}

impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n1, u1, l1), Self::Number(n2, u2, l2)) => {
                n1 == n2 && u1 == u2 && l1 == l2
            }
            _ => false,
        }
    }
}

impl Eq for Schema {}

impl Hash for Schema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Empty(l) => {
                1.hash(state);
                l.hash(state);
            }
            Self::Symbol(c) => {
                2.hash(state);
                c.hash(state);
            }
            Self::Number(n, u, l) => {
                3.hash(state);
                n.hash(state);
                u.hash(state);
                l.hash(state);
            }
            Self::Gear => {
                4.hash(state);
            }
        }
    }
}
