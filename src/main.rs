mod game_engine;
mod grid;
mod ui;

use crate::{
    game_engine::GameState,
    grid::{Grid, Pattern},
};
use std::sync::mpsc::channel;

fn main() {
    let (input_event_tx, input_event_rx) = channel();
    let (game_event_tx, game_event_rx) = channel();
    let initial_game_state = GameState::new(Grid::new(Pattern::Random(10)));
    let ui = ui::start_ui_renderer(&initial_game_state, game_event_rx);
    let game_loop = game_engine::start_loop(initial_game_state, 20, input_event_rx, game_event_tx);
    let _ = ui::start_input_listener(input_event_tx);

    game_loop.join().unwrap();
    ui.join().unwrap();
}
