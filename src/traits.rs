use crate::outputers::Outputer;

pub trait Cell {
    fn is_alive(&self) -> bool;
    fn is_dead(&self) -> bool;
}

pub trait Field {
    fn new<O: 'static + Outputer>(outputer: O) -> Self;
    fn new_randomized<O: 'static + Outputer>(outputer: O) -> Self;
    fn to_live(&mut self);
    fn to_output_format() -> Vec<Vec<impl Cell>>;
}
