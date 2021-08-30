pub mod traits;
use constants::{H, W};

type Field = Vec<Cell>;

fn render_field(field: &Field) {
    let mut row = String::from("|");
    for i in 0..(W * H) {
        if field[i].0 {
            row.push_str("■|");
        } else {
            row.push_str("_|");
        }
        if (i + 1) % W == 0 {
            println!("{}", row);
            row = String::from("|");
        }
    }
}

fn randomized_field() -> Field {
    let mut rng = rand::thread_rng();
    let mut field: Field = Vec::new();
    for _ in 0..(W * H) {
        field.push(Cell(rng.gen::<bool>()));
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

    fn update(&mut self, field: &Field, position: usize) {
        let mut neighbours_count = 0;
        let prev_line = position - W;
        let next_line = position + W;

        if prev_line > 0 {
            if field[prev_line - 1].0 {
                neighbours_count += 1;
            }
            if field[prev_line].0 {
                neighbours_count += 1;
            }
            if field[prev_line + 1].0 {
                neighbours_count += 1;
            }
        }

        if next_line < W * H {
            if field[prev_line - 1].0 {
                neighbours_count += 1;
            }
            if field[prev_line].0 {
                neighbours_count += 1;
            }
            if field[prev_line + 1].0 {
                neighbours_count += 1;
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
