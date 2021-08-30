// Outputers - different ways to output result: CLI, GUI, WASM
pub trait Outputer {
    fn output(&self);
}

pub struct CLIOutputer;

impl Outputer for CLIOutputer {
    fn output(&self) {}
}
