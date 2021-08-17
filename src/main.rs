use rand::Rng;
use bool;

const W: i32 = 5;
const H: i32 = 5;

struct Cell(bool);

impl Cell {
    fn switch(&mut self) { self.0 = !self.0 }

    fn update_cell(&mut self, field: &Field) {
        for row_offset in -1..1 {
            for col_offset in -1..1 {

            }
        }
    }
}

// type Field = [[Cell; W as usize]; H as usize]; TODO
// https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
type Field = Vec<Vec<Cell>>;

fn render_field(field: &Field) {
    for i in field {
        let mut row = String::from("|");
        for cell in i {
            if cell.0 {
                row.push_str("■|");
            } else {
                row.push_str("_|");
            }
        }
        println!("{}", row);
    }
}

fn randomized_field() -> Field {
    let mut rng = rand::thread_rng();
    let mut field: Field = Vec::new();
    for _ in 0..W {
        let mut row = Vec::new();
        for _ in 0..H {
            row.push(Cell (rng.gen::<bool>()));
        }
        field.push(row);
    }

    field
}

fn update_field(field: &Field) {
    for row in field {
        for cell in row {
            cell.update(&field);
        }
    }
}


fn main() {
    // render_field(field);
    let mut field = randomized_field();
    loop {
        render_field(&field);
        update_field(&field);
    }
}