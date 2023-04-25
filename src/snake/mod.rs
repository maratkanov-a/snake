#[cfg(test)]
mod snake_test;

use crate::Pos;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub type Position = (usize, usize, Direction);

pub struct Snake {
    positions: Vec<Position>,
}

impl Snake {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("snake: please provide non 0 size")
        }

        let mut positions = Vec::with_capacity(size);
        for i in 0..size {
            positions.push((3 + i, 3, Direction::Up))
        }

        Self { positions }
    }

    pub fn make_move(&mut self) {
        let (x, y, d) = self.positions.get(0).cloned().unwrap();

        self.positions.pop();

        let np = match d {
            Direction::Up => (x - 1, y, d),
            Direction::Right => (x, y + 1, d),
            Direction::Down => (x + 1, y, d),
            Direction::Left => (x, y - 1, d),
        };

        // prepend
        self.positions.insert(0, np)
    }

    pub fn change_direction(&mut self, nd: Direction) {
        let (px, py, pd) = self.positions.get(0).cloned().unwrap();

        let np = match pd {
            Direction::Up => match nd {
                Direction::Left => Some((px, py - 1, nd)),
                Direction::Right => Some((px, py + 1, nd)),
                _ => None,
            },
            Direction::Right => match nd {
                Direction::Up => Some((px - 1, py, nd)),
                Direction::Down => Some((px + 1, py, nd)),
                _ => None,
            },
            Direction::Down => match nd {
                Direction::Left => Some((px, py - 1, nd)),
                Direction::Right => Some((px, py + 1, nd)),
                _ => None,
            },
            Direction::Left => match nd {
                Direction::Up => Some((px - 1, py, nd)),
                Direction::Down => Some((px + 1, py, nd)),
                _ => None,
            },
        };

        if !np.is_some() {
            return;
        }

        self.positions.pop();
        self.positions.insert(0, np.unwrap())
    }

    fn get_pos(&self, i: usize, p: &Position) -> Box<Pos> {
        let (x, y, d) = p.clone();

        let pos = match d {
            Direction::Up if i == 0 => (x, y, '^'),
            Direction::Right if i == 0 => (x, y, '>'),
            Direction::Down if i == 0 => (x, y, 'v'),
            Direction::Left if i == 0 => (x, y, '<'),
            Direction::Up | Direction::Down => (x, y, '|'),
            Direction::Left | Direction::Right => (x, y, '-'),
        };

        return Box::new(pos);
    }

    pub fn get_positions(&self) -> &Vec<Position> {
        &self.positions
    }

    pub fn grow(&mut self) {
        let (x, y, d) = self.positions.last().unwrap();
        let (x, y, d) = (*x, *y, *d);

        let np = match d {
            Direction::Up => (x + 1, y, d),
            Direction::Right => (x, y - 1, d),
            Direction::Down => (x - 1, y, d),
            Direction::Left => (x, y + 1, d),
        };

        self.positions.push(np)
    }
}

impl crate::Drawable for Snake {
    fn pos(&self) -> Vec<Pos> {
        let mut pos = Vec::with_capacity(self.positions.len());

        for (i, p) in self.positions.iter().enumerate() {
            let p = self.get_pos(i, p);
            pos.push(*p)
        }

        pos
    }
}

impl AsRef<Snake> for Snake {
    fn as_ref(&self) -> &Snake {
        self
    }
}
