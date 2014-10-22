use std::collections::HashMap;
use std::str::CharSplits;

use collections::hash::Hash;


pub trait PrimaryKey<T> {
    fn primary_key(&self) -> T;
}

pub trait FromRow {
    fn from_row(mut row: CharSplits<char>) -> Self;
}

#[deriving(Show)]
pub struct Table<Data, Index: Eq + Hash> {
    rows: Vec<Data>,
    index: HashMap<Index, uint>
}

impl<Data: PrimaryKey<Index> + Clone, Index: Eq + Hash> Table<Data, Index> {
    pub fn new() -> Table<Data, Index> {
        Table {
            rows: Vec::new(),
            index: HashMap::new()
        }
    }

    pub fn insert(&mut self, data: Data) {
        self.rows.push(data.clone());
        self.index.insert(data.primary_key(), self.rows.len() - 1);
    }

    pub fn lookup(&self, index_elem: Index) -> &Data {
        &self.rows[self.index[index_elem]]
    }

    pub fn lookup_mut(&mut self, index_elem: Index) -> &mut Data {
        self.rows.get_mut(self.index[index_elem])
    }

    pub fn size(&self) -> uint {
        self.rows.len()
    }
}
