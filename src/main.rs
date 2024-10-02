mod grid;
mod ui;
mod game_engine;

use crate::grid::Grid;
use crate::ui::Ui;

use std::sync::{Arc, Mutex};
use std::time::Duration;


fn main() {
    let grid = Arc::new(Mutex::new(Grid::new(5)));
    let ui = Arc::new(Mutex::new(Ui::new()));
    let game_loop = game_engine::start_loop(ui, grid, Duration::from_secs(1));

    game_loop.join().unwrap();
}
