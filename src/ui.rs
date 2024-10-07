use crate::{
    game_engine::{GameEvent, GameState},
    grid::{self, Grid, Pattern},
};

use std::fmt;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;
use termion::{clear, color, cursor};

const UI_WIDTH: usize = grid::WIDTH * 2;
const CONTROLS_WIDTH: usize = 1 + 12 + 2 + 6 + 16 + 2 + 14;
const CONTROLS_FILLER_WIDTH: usize = UI_WIDTH - CONTROLS_WIDTH;

pub enum InputEvent {
    GridClicked(Coordinates),
    TogglePause,
    IncrementUpdateFrequence,
    DecrementUpdateFrequence,
    Reset(Pattern),
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
        let mut out = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();
        writeln!(out, "{}", cursor::Hide).unwrap();

        Ui {
            out: Box::from(MouseTerminal::from(out)),
        }
    }

    pub fn render(&mut self, game_state: &GameState) {
        write!(self.out, "{}\r\n", cursor::Goto(1, 1)).unwrap();
        self.render_grid(&game_state.grid);
        self.render_info(game_state);
        write!(self.out, "\r\n").unwrap();
        self.render_controls(game_state);
        write!(self.out, "\r\n").unwrap();
        self.render_preset_options();
        write!(self.out, "{}", clear::AfterCursor).unwrap();
        self.out.flush().unwrap();
    }

    fn render_grid(&mut self, grid: &Grid) {
        let default_color = color::Fg(color::Reset);
        let alive_color = color::Fg(color::LightCyan);

        // for (y, row) in grid.cells.iter().enumerate() {
        //     for (x, alive) in row.iter().enumerate() {
        //         if *alive {
        //             let count = grid.count_living_neighbours(x, y);
        //             write!(self.out, "{alive_color} {count}").unwrap();
        //         } else {
        //             write!(self.out, "{default_color} -").unwrap();
        //         }
        //     }
        //     write!(self.out, "\r\n").unwrap();
        // }

        for row in grid.cells.iter() {
            for alive in row.iter() {
                if *alive {
                    write!(self.out, "{alive_color} ■").unwrap();
                } else {
                    write!(self.out, "{default_color} □").unwrap();
                }
            }
            write!(self.out, "\r\n").unwrap();
        }
    }

    fn render_info(&mut self, game_state: &GameState) {
        let gen = format!("generation: {}", game_state.generation);
        let speed = format!("speed: {}x", game_state.update_frequence);
        write!(self.out, " {gen}{speed:>0$}\r\n", UI_WIDTH - gen.len() - 1).unwrap();
    }

    fn render_controls(&mut self, game_state: &GameState) {
        let default_color = color::Fg(color::Reset);
        let inactive_color = color::Fg(color::LightBlack);
        let hotkey_color = color::Fg(color::Cyan);
        let filler = format!("{0:1$}", " ", CONTROLS_FILLER_WIDTH);

        write!(self.out, " ").unwrap();

        if game_state.paused {
            if game_state.generation == 0 {
                write!(self.out, "{hotkey_color}space{default_color} start ").unwrap();
            } else {
                write!(self.out, "{hotkey_color}space{default_color} resume").unwrap();
            }
        } else {
            write!(self.out, "{hotkey_color}space{default_color} pause ").unwrap();
        }

        write!(self.out, "  ").unwrap();
        write!(self.out, "{hotkey_color}q{default_color} quit").unwrap();
        write!(self.out, "{filler}").unwrap();

        if game_state.is_max_update_frequence {
            write!(self.out, "{inactive_color}+ increase speed{default_color}").unwrap();
        } else {
            write!(self.out, "{hotkey_color}+{default_color} increase speed").unwrap();
        }

        write!(self.out, "  ").unwrap();

        if game_state.is_min_update_frequence {
            write!(self.out, "{inactive_color}- reduce speed{default_color}").unwrap();
        } else {
            write!(self.out, "{hotkey_color}-{default_color} reduce speed").unwrap();
        }

        write!(self.out, "\r\n").unwrap();
    }

    fn render_preset_options(&mut self) {
        let default_color = color::Fg(color::Reset);
        let default_len = default_color.to_string().len();
        let hotkey_color = color::Fg(color::Cyan);
        let hotkey_len = hotkey_color.to_string().len();
        let left_pad = format!("{:1$}", " ", hotkey_len + default_len);
        let f1 = format!("{left_pad}{hotkey_color}F1{default_color} Reset (random)             ");
        let f2 = format!("{left_pad}{hotkey_color}F2{default_color} Preset: Gosper's glider gun");

        write!(self.out, " {:^1$}", f1, UI_WIDTH).unwrap();
        write!(self.out, "\r\n").unwrap();
        write!(self.out, " {:^1$}", f2, UI_WIDTH).unwrap();
    }
}

pub fn start_ui_renderer(initial_game_state: &GameState, event_stream: Receiver<GameEvent>) -> JoinHandle<()> {
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
            Key::F(1) => Some(InputEvent::Reset(Pattern::Random(10))),
            Key::F(2) => Some(InputEvent::Reset(Pattern::GosperGliderGun)),
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
        if x < grid::WIDTH && y < grid::HEIGHT {
            Some(Coordinates { x, y })
        } else {
            // out of bounds
            None
        }
    }
}
