use bool;
use rand::Rng;

const W: i32 = 5;
const H: i32 = 5;

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
            row.push(Cell(rng.gen::<bool>()));
        }
        field.push(row);
    }

    field
}

struct Cell(bool);

impl std::cmp::PartialEq for Cell {
    fn ne(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn eq(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl Cell {
    fn switch(&mut self) {
        self.0 = !self.0
    }

    fn update(&mut self, field: &Field, position: (i32, i32)) {
        let mut neighbours_count = 0;
        for row_offset in -1..1 {
            for col_offset in -1..1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let new_row = row_offset + position.0;
                if new_row < 0 || new_row >= H {
                    continue;
                }

                let new_col = col_offset + position.1;
                if new_col < 0 || new_col >= W {
                    continue;
                }

                if field[new_row as usize][new_col as usize].0 {
                    neighbours_count += 1;
                }
            }
        }

        if self.0 {
            if !(neighbours_count <= 1 || neighbours_count >= 4) {
                return;
            }
        } else if neighbours_count == 3 {
            self.0 = true;
            return;
        }

        self.0 = false;
    }
}

fn main() {
    // render_field(field);
    let mut field = randomized_field();
    loop {
        render_field(&field);

        for (row_ind, row) in field.iter_mut().enumerate() {
            for (col_ind, cell) in row.iter_mut().enumerate() {
                cell.update(&field, (row_ind as i32, col_ind as i32));
            }
        }
    }
}
