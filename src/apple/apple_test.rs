#[cfg(test)]
mod test {
    use crate::apple::*;
    use crate::snake::Direction;

    #[test]
    #[should_panic(expected = "apple: please provide board size")]
    fn x_zero_apple() {
        Apple::new(0, 10, &vec![]);
    }

    #[test]
    #[should_panic(expected = "apple: please provide board size")]
    fn y_zero_apple() {
        Apple::new(10, 0, &vec![]);
    }

    #[test]
    fn no_positions_expect_any() {
        let a = Apple::new(10, 10, &vec![]);
        assert!(a.x > 0 && a.x < 10);
        assert!(a.y > 0 && a.y < 10)
    }

    #[test]
    fn one_positions_expect_ne() {
        let a = Apple::new(10, 10, &vec![(2, 2, Direction::Up)]);
        assert!(!(a.x == 2 && a.y == 2));
    }

    #[test]
    fn all_positions_expect_zeroes() {
        let a = Apple::new(3, 3, &vec![(1, 1, Direction::Up)]);
        assert!(a.x == 0 && a.y == 0);
    }
}
