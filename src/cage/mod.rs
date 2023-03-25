use crate::Pos;

pub struct Cage {
    x: usize,
    y: usize,
}

impl Cage {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl crate::Drawable for Cage {
    fn pos(&self) -> Vec<Pos> {
        let mut v: Vec<Pos> = Vec::with_capacity(self.x * self.y);

        for i in 0..self.x {
            for j in 0..self.y {
                if i == 0 || i == self.x - 1 {
                    v.push((i, j, '☐'));
                    continue;
                }

                if (i > 0 && j == 0) || (i > 0 && j == self.y - 1) {
                    v.push((i, j, '☐'));
                    continue;
                }
            }
        }

        v
    }
}
