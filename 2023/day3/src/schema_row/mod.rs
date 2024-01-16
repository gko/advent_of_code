use std::ops::Index;

use crate::schema::Schema;

#[derive(Debug)]
pub struct SchemaRow(pub Vec<Schema>);

impl Iterator for SchemaRow {
    type Item = Schema;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl ExactSizeIterator for SchemaRow {
    fn len(&self) -> usize {
        self.0.iter().fold(0, |acc, i| acc + i.len())
    }
}

impl Index<usize> for SchemaRow {
    type Output = Schema;

    fn index(&self, index: usize) -> &Self::Output {
        let mut overall_length: usize = 0;
        for (i, item) in self.0.iter().enumerate() {
            if overall_length + item.len() > index {
                return &self.0[i];
            } else {
                overall_length += item.len();
            }
        }

        &self.0.last().unwrap_or(&Schema::Empty(0))
    }
}
