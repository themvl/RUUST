pub mod console_game;
mod game_state;
pub mod websocket_game;

fn main() {
    // console_game::run();
    websocket_game::run();
}
