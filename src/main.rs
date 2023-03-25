use snake_game::game;

fn main() {
    let game = game::Game::new(10, 10);
    game::Game::start(game);
}
