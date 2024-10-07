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

    fn try_increment_update_frequence(&mut self) -> bool {
        if self.is_max_update_frequence {
            false
        } else if self.update_frequence < 1.0 {
            self.adjust_update_frequence(0.2);
            true
        } else {
            self.adjust_update_frequence(0.5);
            true
        }
    }

    fn try_decrement_update_frequence(&mut self) -> bool {
        if self.is_min_update_frequence {
            false
        } else if self.update_frequence <= 1.0 {
            self.adjust_update_frequence(-0.2);
            true
        } else {
            self.adjust_update_frequence(-0.5);
            true
        }
    }

    fn adjust_update_frequence(&mut self, by: f32) -> f32 {
        let new_val = self.update_frequence + by;
        self.update_frequence = (new_val * 10.0).round() / 10.0;
        self.is_min_update_frequence = self.update_frequence <= MIN_UPDATE_FREQUENCE;
        self.is_max_update_frequence = self.update_frequence >= MAX_UPDATE_FREQUENCE;
        self.update_frequence
    }
}

pub fn start_loop(
    initial_game_state: GameState,
    ticks_per_sec: u32,
    input_stream: Receiver<InputEvent>,
    event_stream: Sender<GameEvent>,
) -> JoinHandle<()> {
    let sleep_duration = Duration::from_millis(1000 / ticks_per_sec as u64);
    let mut tick_counter = 0;
    let mut game_state = initial_game_state;
    let mut ticks_per_update = calc_ticks_per_update(ticks_per_sec, &game_state.update_frequence);

    spawn(move || loop {
        // check for input and handle it if there is any
        let mut state_updated = match input_stream.try_recv() {
            Ok(event) => match event {
                InputEvent::GridClicked(c) => game_state.grid.try_toggle_cell(c.x, c.y),
                InputEvent::TogglePause => {
                    game_state.paused = !game_state.paused;
                    true
                }
                InputEvent::IncrementUpdateFrequence => game_state.try_increment_update_frequence(),
                InputEvent::DecrementUpdateFrequence => game_state.try_decrement_update_frequence(),
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
            Err(TryRecvError::Empty) => false,
        };

        // handle tick if not paused
        if !game_state.paused {
            // advance to next generation if it's time
            if tick_counter >= ticks_per_update {
                let update_count = game_state.grid.next_generation();
                if update_count == 0 {
                    // automatically pause game if we're "stuck"
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
            event_stream.send(GameEvent::StateUpdated(game_state.clone())).unwrap();
            ticks_per_update = calc_ticks_per_update(ticks_per_sec, &game_state.update_frequence);
        }

        sleep(sleep_duration);
    })
}

fn calc_ticks_per_update(ticks_per_sec: u32, update_frequence: &f32) -> u32 {
    (ticks_per_sec as f32 / update_frequence).round() as u32
}
