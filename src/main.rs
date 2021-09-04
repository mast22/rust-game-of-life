mod constants;
mod hash_map_field;
mod traits;
mod outputers;

use hash_map_field::HashMapField;
use traits::Field;
use outputers::CLIOutputer;


fn main() {
    let mut field = HashMapField::new_randomized(CLIOutputer {});

    field.to_live();
}
