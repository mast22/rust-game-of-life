use crate::outputers::Outputer;

pub trait Cell {
    fn is_alive() -> bool;
    fn is_dead() -> bool;
    fn get_x() -> i32;
    fn get_y() -> i32;
}

pub trait Field {
    fn new() -> Self;
    fn new_randomized() -> Self;
    fn to_live(&mut self, outputer: impl Outputer);
}
