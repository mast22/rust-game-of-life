use rand::Rng;

// HashMap Field implementation
// Position of a cell is defined by
use crate::constants::{H, W};
use crate::outputers::Outputer;
use crate::traits::{Cell, Field};
use std::collections::HashMap;

pub struct HashMapCell(bool);

impl Cell for HashMapCell {
    fn is_alive(&self) -> bool {
        self.0
    }

    fn is_dead(&self) -> bool {
        !self.0
    }
}

pub struct HashMapField {
    cells: HashMap<(usize, usize), HashMapCell>,
}

impl Field for HashMapField {
    fn new<O: 'static + Outputer>(outputer: O) -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn new_randomized<O: 'static + Outputer>(outputer: O) -> Self {
        let mut rng = rand::thread_rng();

        let mut new_field = HashMap::new();

        for i in 0..H {
            for j in 0..W {
                new_field.insert((i, j), HashMapCell(rng.gen()));
            }
        }
        Self { cells: new_field }
    }

    fn to_live(&mut self) {
        todo!()
    }
}
