use std::{
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

use crate::{grid::Grid, ui::InputEvent};

const MIN_UPDATE_FREQUENCE: f32 = 0.2;
const MAX_UPDATE_FREQUENCE: f32 = 5.0;

pub enum GameEvent {
    StateUpdated(GameState),
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub grid: Grid,
    pub update_frequence: f32,
    pub is_min_update_frequence: bool,
    pub is_max_update_frequence: bool,
    pub paused: bool,
    pub generation: u64,
}

impl GameState {
    pub fn new(grid: Grid) -> Self {
        GameState {
            grid,
            update_frequence: 1.0,
            is_min_update_frequence: false,
            is_max_update_frequence: false,
            paused: true,
            generation: 0,
        }
    }

    fn adjust_update_frequence(&mut self, by: f32) {
        let new_val = self.update_frequence + by;
        self.update_frequence = (new_val * 10.0).round() / 10.0;
    }
}

pub fn start_loop(
    initial_game_state: GameState,
    tick_rate: u32,
    input_stream: Receiver<InputEvent>,
    event_stream: Sender<GameEvent>,
) -> JoinHandle<()> {
    let sleep_duration = Duration::from_millis(1000 / tick_rate as u64);
    let mut tick_counter = 0;
    let mut game_state = initial_game_state;
    let mut ticks_per_update = (tick_rate as f32 / game_state.update_frequence) as u32;

    spawn(move || loop {
        // check for input and handle it if there is any
        let mut state_updated = match input_stream.try_recv() {
            Ok(event) => match event {
                InputEvent::GridClicked(c) => {
                    game_state.grid.toggle_cell(c.x, c.y);
                    true
                }
                InputEvent::TogglePause => {
                    game_state.paused = !game_state.paused;
                    true
                }
                InputEvent::IncrementUpdateFrequence => {
                    let amount = if game_state.update_frequence >= MAX_UPDATE_FREQUENCE {
                        0.0
                    } else if game_state.update_frequence < 1.0 {
                        0.2
                    } else {
                        0.5
                    };

                    if amount != 0.0 {
                        game_state.adjust_update_frequence(amount);
                        ticks_per_update = (tick_rate as f32 / game_state.update_frequence) as u32;
                        true
                    } else {
                        false
                    }
                }
                InputEvent::DecrementUpdateFrequence => {
                    let amount = if game_state.update_frequence <= MIN_UPDATE_FREQUENCE {
                        0.0
                    } else if game_state.update_frequence <= 1.0 {
                        -0.2
                    } else {
                        -0.5
                    };

                    if amount != 0.0 {
                        game_state.adjust_update_frequence(amount);
                        ticks_per_update = (tick_rate as f32 / game_state.update_frequence) as u32;
                        true
                    } else {
                        false
                    }
                }
                InputEvent::Reset(pattern) => {
                    game_state.grid.reset(pattern);
                    true
                }
                InputEvent::Quit => {
                    event_stream.send(GameEvent::Quit).unwrap();
                    return;
                }
            },
            Err(TryRecvError::Disconnected) => {
                event_stream.send(GameEvent::Quit).unwrap();
                return;
            }
            _ => false,
        };

        // go to next generation if it's time
        if !game_state.paused {
            if tick_counter >= ticks_per_update {
                let update_count = game_state.grid.next_generation();
                if update_count == 0 {
                    // pause game if we're "stuck"
                    game_state.paused = true;
                }
                game_state.generation += 1;
                state_updated = true;
                tick_counter = 0;
            } else {
                tick_counter += 1;
            }
        }

        if state_updated {
            event_stream
                .send(GameEvent::StateUpdated(game_state.clone()))
                .unwrap();
        }

        sleep(sleep_duration);
    })
}
