use crate::Drawable;

pub struct Field {
    x: usize,
    y: usize,
    field: Vec<Vec<char>>,
}

impl Field {
    pub fn new(x: usize, y: usize) -> Self {
        let mut field: Vec<Vec<char>> = Vec::with_capacity(x * y);
        for _ in 0..x {
            let mut v = vec![];
            for _ in 0..y {
                v.push(' ')
            }

            field.push(v)
        }

        Self { field, x, y }
    }

    pub fn draw(&mut self, elements: Vec<&dyn Drawable>) {
        for e in elements.iter() {
            for (x, y, symbol) in e.pos().iter() {
                self.field[*x][*y] = *symbol
            }
        }

        let mut result: String = String::new();
        for (i, v) in self.field.iter().enumerate() {
            result.push_str(&termion::cursor::Goto(1, (i + 1) as u16).to_string());
            result.push_str(&*v.iter().collect::<String>());
            result.push_str("\n");
        }

        println!("{}", result);

        for i in 0..self.x {
            for j in 0..self.y {
                self.field[i][j] = ' ';
            }
        }
    }
}
