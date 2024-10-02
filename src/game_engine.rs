use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::{grid::Grid, ui::Ui};

pub fn start_loop(
    ui: Arc<Mutex<Ui>>,
    grid: Arc<Mutex<Grid>>,
    update_frequence: Duration,
) -> JoinHandle<()> {
    let ui = Arc::clone(&ui);
    let grid = Arc::clone(&grid);

    spawn(move || loop {
        {
            let mut grid = grid.lock().unwrap();
            grid.next_generation();
            let mut ui = ui.lock().unwrap();
            ui.draw_grid(&grid);
        }
        sleep(update_frequence);
    })
}
