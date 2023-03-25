pub mod apple;
pub mod cage;
pub mod field;
pub mod game;
pub mod snake;

pub type Pos = (usize, usize, char);

pub trait Drawable {
    fn pos(&self) -> Vec<Pos>;
}
