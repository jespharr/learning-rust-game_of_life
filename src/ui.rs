use crate::game_engine::{GameEvent, GameState};
use crate::grid::{self, Grid};

use std::fmt;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{clear, color, cursor};

pub enum InputEvent {
    GridClicked(Coordinates),
    TogglePause,
    IncrementUpdateFrequence,
    DecrementUpdateFrequence,
    Quit,
}

pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{},{}}}", self.x, self.y)
    }
}

pub struct Ui {
    out: Box<dyn Write + Send>,
}

impl Ui {
    pub fn new() -> Self {
        let mut out = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        writeln!(out, "{}", cursor::Hide).unwrap();

        Ui {
            out: Box::from(MouseTerminal::from(out)),
        }
    }

    pub fn render(&mut self, game_state: &GameState) {
        write!(self.out, "{}", cursor::Goto(1, 1)).unwrap();
        write!(self.out, "\r\n").unwrap();
        self.render_grid(&game_state.grid);
        write!(self.out, "{}", clear::UntilNewline).unwrap();
        write!(self.out, "\r\n").unwrap();
        self.render_info(game_state);
        write!(self.out, "{}", clear::UntilNewline).unwrap();
        write!(self.out, "\r\n").unwrap();
        self.render_controls(game_state);
        write!(self.out, "{}", clear::AfterCursor).unwrap();
        self.out.flush().unwrap();
    }

    fn render_grid(&mut self, grid: &Grid) {
        let default_color = color::Fg(color::Reset);
        let alive_color = color::Fg(color::Cyan);

        for (y, row) in grid.cells.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                if *alive {
                    let count = grid.count_living_neighbours(x, y);
                    write!(self.out, "{alive_color} {count}").unwrap();
                } else {
                    write!(self.out, "{default_color} 0").unwrap();
                }
            }
            write!(self.out, "\r\n").unwrap();
        }
    }

    fn render_info(&mut self, game_state: &GameState) {
        let gen = game_state.generation;
        write!(self.out, "generation: {gen}   ").unwrap();

        let speed = game_state.update_frequence;
        write!(self.out, "speed: {speed}x").unwrap();
    }

    fn render_controls(&mut self, game_state: &GameState) {
        write!(self.out, "q = quit   ").unwrap();
        if game_state.paused {
            write!(self.out, "space = resume   ").unwrap();
        } else {
            write!(self.out, "space = pause    ").unwrap();
        }
        if game_state.is_max_update_frequence {
            write!(self.out, "                     ").unwrap();
        } else {
            write!(self.out, "+ = increase speed   ").unwrap();
        }
        if game_state.is_min_update_frequence {
            write!(self.out, "                   ").unwrap();
        } else {
            write!(self.out, "- = reduce speed   ").unwrap();
        }
    }
}

pub fn start_ui_renderer(
    initial_game_state: &GameState,
    event_stream: Receiver<GameEvent>,
) -> JoinHandle<()> {
    let mut ui = Ui::new();
    ui.render(initial_game_state);

    spawn(move || loop {
        match event_stream.recv() {
            Ok(event) => match event {
                GameEvent::StateUpdated(game_state) => ui.render(&game_state),
                GameEvent::Quit => return,
            },
            Err(_) => return,
        }
    })
}

pub fn start_input_listener(input_stream: Sender<InputEvent>) -> JoinHandle<()> {
    spawn(move || {
        let mut events = stdin().events();
        loop {
            // check for valid input and send it to game loop
            if let Some(Ok(event)) = events.next() {
                if let Some(event) = translate_to_input_event(event) {
                    input_stream.send(event).unwrap();
                }
            }
        }
    })
}

fn translate_to_input_event(event: Event) -> Option<InputEvent> {
    match event {
        Event::Mouse(MouseEvent::Press(button, x, y)) if button == MouseButton::Left => {
            if let Some(coords) = translate_to_grid_coordinates(x, y) {
                Some(InputEvent::GridClicked(coords))
            } else {
                None
            }
        }
        Event::Key(key) => match key {
            Key::Char(' ') => Some(InputEvent::TogglePause),
            Key::Char('+') => Some(InputEvent::IncrementUpdateFrequence),
            Key::Char('-') => Some(InputEvent::DecrementUpdateFrequence),
            Key::Char('q') => Some(InputEvent::Quit),
            _ => None,
        },
        _ => None,
    }
}

fn translate_to_grid_coordinates(x: u16, y: u16) -> Option<Coordinates> {
    // grid is "padded" by 1 and mouse coordinates are 1-based but grid is 0-based
    let x = (x - 2) as usize;
    let y = (y - 2) as usize;

    if x % 2 == 1 {
        // cells have a horizontal spacing of 1 so odd x-coordinates are between cells
        None
    } else {
        let x = x / 2;
        if x < grid::GRID_WIDTH && y < grid::GRID_HEIGHT {
            Some(Coordinates { x, y })
        } else {
            // out of bounds
            None
        }
    }
}
