// HashMap Field implementation
// Position of a cell is defined by
use crate::traits::{Cell, Field};
use crate::outputers::{Outputer};
use std::collections::HashMap;

pub struct HashMapCell(bool);

impl Cell for HashMapCell {
    fn is_alive() -> bool {
        todo!()
    }

    fn is_dead() -> bool {
        todo!()
    }

    fn get_x() -> i32 {
        todo!()
    }

    fn get_y() -> i32 {
        todo!()
    }
}

pub struct HashMapField {
    cells: HashMap<(usize, usize), HashMapCell>,
}

impl Field for HashMapField {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn new_randomized() -> Self {
        todo!()
    }

    fn to_live(&mut self, ouputer: impl Outputer) {
        todo!()
    }
}
