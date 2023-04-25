#[cfg(test)]
mod apple_test;

use rand::Rng;
use std::collections::HashSet;
use std::vec;

use crate::snake::Position;
use crate::{Drawable, Pos};

pub struct Apple {
    x: usize,
    y: usize,
}

impl Apple {
    pub fn new(x: usize, y: usize, ps: &Vec<Position>) -> Self {
        if x == 0 || y == 0 {
            panic!("apple: please provide board size")
        }

        let mut rng = rand::thread_rng();

        let mut nx: usize = 0;
        let mut ny: usize = 0;
        let mut position_set = HashSet::new();
        for v in ps {
            position_set.insert((v.0, v.1));
        }

        for _ in 0..x * y {
            let gx = rng.gen_range(1..x - 1);
            let gy = rng.gen_range(1..y - 1);

            if position_set.contains(&(gx, gy)) {
                continue;
            }

            nx = gx;
            ny = gy;

            break;
        }

        Self { x: nx, y: ny }
    }

    pub fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl Drawable for Apple {
    fn pos(&self) -> Vec<Pos> {
        let mut v = Vec::with_capacity(1);
        v.push((self.x, self.y, '•'));
        v
    }
}
