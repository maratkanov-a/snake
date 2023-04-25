mod messages;

use std::io::{stdin, stdout, Stdout, Write};
use std::sync::mpsc;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::snake::Direction;
use crate::{apple, cage, field, snake};

enum Status {
    Started,
    Paused,
    Ended,
}

pub struct Game {
    x: usize,
    y: usize,

    field: field::Field,
    snake: snake::Snake,
    cage: cage::Cage,
    apple: apple::Apple,

    status: Status,
}

impl Game {
    pub fn new(x: usize, y: usize) -> Self {
        let snake = snake::Snake::new(2);
        let apple = {
            let sps = snake.get_positions();
            apple::Apple::new(x, y, sps)
        };

        Self {
            x,
            y,
            snake,
            apple,
            field: field::Field::new(x, y),
            cage: cage::Cage::new(x, y),
            status: Status::Paused,
        }
    }

    fn clear(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::clear::All
        )
        .unwrap();
    }

    fn display(&self, stdout: &mut RawTerminal<Stdout>) {
        stdout.flush().unwrap();
    }

    fn has_intersection(&mut self) {
        let (x, y, _) = self.snake.get_positions().get(0).cloned().unwrap();

        if x == self.x - 1 || y == self.y - 1 || x == 0 || y == 0 {
            self.status = Status::Ended;
            return;
        }

        let (apx, apy) = self.apple.get_position();
        if x == apx && y == apy {
            self.snake.grow();

            let sps = self.snake.get_positions();
            self.apple = apple::Apple::new(self.x, self.y, sps)
        }
    }

    fn draw(&mut self, stdout: &mut RawTerminal<Stdout>) {
        self.clear(stdout);
        self.field.draw(vec![&self.cage, &self.snake, &self.apple]);
        self.display(stdout);
    }

    fn message(&self, stdout: &mut RawTerminal<Stdout>, ms: Vec<&str>) {
        self.clear(stdout);
        let mut result: String = String::new();
        for (i, m) in ms.iter().enumerate() {
            result.push_str(&termion::cursor::Goto(1, (i + 1) as u16).to_string());
            result.push_str(*m);
            result.push_str("\n");
        }
        println!("{}", result);
        self.display(stdout);
    }

    pub fn start(mut self) {
        let (keys_sender, keys_receiver): (mpsc::Sender<Direction>, mpsc::Receiver<Direction>) =
            mpsc::channel();
        let (menu_sender, menu_receiver): (mpsc::Sender<Status>, mpsc::Receiver<Status>) =
            mpsc::channel();
        let (status_sender, status_receiver): (mpsc::Sender<Status>, mpsc::Receiver<Status>) =
            mpsc::channel();

        let mut stdout = stdout().into_raw_mode().unwrap();

        // initial draw
        self.message(&mut stdout, messages::WELCOME.to_vec());
        // end of initial draw

        let ms = menu_sender.clone();
        // every tick thread
        let game = std::thread::spawn(move || {
            std::thread::park();

            self.draw(&mut stdout);

            loop {
                if let Ok(event) = status_receiver.try_recv() {
                    match event {
                        Status::Started => self.status = Status::Started,
                        Status::Paused => {
                            self.message(&mut stdout, messages::PAUSED.to_vec());
                            std::thread::park();
                            self.draw(&mut stdout);
                            continue;
                        }
                        Status::Ended => {
                            self.message(&mut stdout, messages::BYE.to_vec());
                            break;
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(500));

                if let Ok(d) = keys_receiver.try_recv() {
                    self.snake.change_direction(d);
                    self.draw(&mut stdout);
                    continue;
                }

                self.snake.make_move();
                self.has_intersection();

                match self.status {
                    Status::Ended => {
                        self.message(&mut stdout, messages::DEATH.to_vec());
                        ms.send(Status::Ended).unwrap();
                        break;
                    }
                    _ => {}
                }

                self.draw(&mut stdout);
            }
        });

        // keyboard handler thread
        std::thread::spawn(move || {
            for c in stdin().keys() {
                match c.unwrap() {
                    Key::Up => keys_sender.send(Direction::Up).unwrap(),
                    Key::Down => keys_sender.send(Direction::Down).unwrap(),
                    Key::Left => keys_sender.send(Direction::Left).unwrap(),
                    Key::Right => keys_sender.send(Direction::Right).unwrap(),

                    Key::Char('s') => {
                        menu_sender.send(Status::Started).unwrap();
                        status_sender.send(Status::Started).unwrap();
                    }
                    Key::Char('p') => {
                        menu_sender.send(Status::Paused).unwrap();
                        status_sender.send(Status::Paused).unwrap();
                    }
                    Key::Esc | Key::Ctrl('c') => {
                        menu_sender.send(Status::Ended).unwrap();
                        status_sender.send(Status::Ended).unwrap();
                        break;
                    }

                    _ => continue,
                }
            }
        });

        let status = std::thread::spawn(move || loop {
            if let Ok(event) = menu_receiver.try_recv() {
                match event {
                    Status::Ended => break,
                    Status::Started => game.thread().unpark(),
                    _ => {}
                }
            }

            continue;
        });

        status.join().unwrap();
    }
}
