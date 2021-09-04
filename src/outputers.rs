// Outputers - different ways to output result: CLI, GUI, WASM
use crate::traits::Cell;

pub trait Outputer {
    fn output(&self, data: Vec<Vec<impl Cell>>);
}

pub struct CLIOutputer;

impl Outputer for CLIOutputer {
    fn output(&self, data: Vec<Vec<impl Cell>>) {
        let mut output = String::new();
        for row in data {
            output.push_str("|");
            for cell in row {
                if cell.is_alive() {
                    output.push_str("■|");
                } else {
                    output.push_str("_|");
                }
            }
            output.push_str("\n");
        }

        println!("{}", output);
    }
}
