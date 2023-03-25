#[cfg(test)]
mod test {
    use crate::snake::{Direction, Snake};

    struct Tc {
        current_direction: Direction,
        new_direction: Direction,
        expected: Direction,
    }

    #[test]
    #[should_panic(expected = "snake: please provide non 0 size")]
    fn zero_size() {
        Snake::new(0);
    }

    #[test]
    fn make_move_same_direction() {
        for d in vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        {
            let mut s = Snake::new(1);
            s.positions[0].2 = *d;
            s.make_move();

            let (_, _, cp) = s.positions[0];

            assert!(cp == *d);
            assert_eq!(s.positions.len(), 1)
        }
    }

    #[test]
    fn change_direction_incorrect_paths() {
        for t in vec![
            Tc {
                current_direction: Direction::Up,
                new_direction: Direction::Up,
                expected: Direction::Up,
            },
            Tc {
                current_direction: Direction::Up,
                new_direction: Direction::Down,
                expected: Direction::Up,
            },
            Tc {
                current_direction: Direction::Right,
                new_direction: Direction::Right,
                expected: Direction::Right,
            },
            Tc {
                current_direction: Direction::Right,
                new_direction: Direction::Left,
                expected: Direction::Right,
            },
            Tc {
                current_direction: Direction::Down,
                new_direction: Direction::Down,
                expected: Direction::Down,
            },
            Tc {
                current_direction: Direction::Down,
                new_direction: Direction::Up,
                expected: Direction::Down,
            },
            Tc {
                current_direction: Direction::Left,
                new_direction: Direction::Left,
                expected: Direction::Left,
            },
            Tc {
                current_direction: Direction::Left,
                new_direction: Direction::Right,
                expected: Direction::Left,
            },
        ] {
            let mut s = Snake::new(1);
            s.positions[0].2 = t.current_direction;

            s.change_direction(t.new_direction);
            println!("{:?}{:?}", s.positions[0].2, t.expected);
            assert!(s.positions[0].2 == t.expected)
        }
    }
    #[test]
    fn change_direction_correct_paths() {
        for t in vec![
            Tc {
                current_direction: Direction::Up,
                new_direction: Direction::Right,
                expected: Direction::Right,
            },
            Tc {
                current_direction: Direction::Up,
                new_direction: Direction::Left,
                expected: Direction::Left,
            },
            Tc {
                current_direction: Direction::Right,
                new_direction: Direction::Up,
                expected: Direction::Up,
            },
            Tc {
                current_direction: Direction::Right,
                new_direction: Direction::Down,
                expected: Direction::Down,
            },
            Tc {
                current_direction: Direction::Down,
                new_direction: Direction::Right,
                expected: Direction::Right,
            },
            Tc {
                current_direction: Direction::Down,
                new_direction: Direction::Left,
                expected: Direction::Left,
            },
            Tc {
                current_direction: Direction::Left,
                new_direction: Direction::Up,
                expected: Direction::Up,
            },
            Tc {
                current_direction: Direction::Left,
                new_direction: Direction::Down,
                expected: Direction::Down,
            },
        ] {
            let mut s = Snake::new(1);
            s.positions[0].2 = t.current_direction;

            s.change_direction(t.new_direction);
            assert!(s.positions[0].2 == t.expected)
        }
    }

    #[test]
    fn grow_test() {
        let mut s = Snake::new(1);
        s.grow();

        assert_eq!(s.positions.len(), 2);

        for p in s.positions.iter() {
            assert_eq!(p.2, Direction::Up)
        }
    }
}
