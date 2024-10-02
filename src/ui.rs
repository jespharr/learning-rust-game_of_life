use crate::grid::Grid;

use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, Mutex};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, cursor};

pub struct Ui {
    out: Arc<Mutex<AlternateScreen<Stdout>>>,
}

impl Ui {
    pub fn new() -> Self {
        let mut out = stdout().into_alternate_screen().unwrap();
        writeln!(out, "{}", cursor::Hide).unwrap();

        Ui {
            out: Arc::new(Mutex::new(out)),
        }
    }

    pub fn draw_grid(&mut self, grid: &Grid) {
        let default_color = color::Fg(color::Reset);
        let alive_color = color::Fg(color::Cyan);
        let mut out = self.out.lock().unwrap();

        writeln!(out, "{}", cursor::Goto(1, 1)).unwrap();

        for (x, row) in grid.cells.iter().enumerate() {
            for (y, alive) in row.iter().enumerate() {
                if *alive {
                    let count = grid.get_living_neighbours_count(x, y);
                    write!(out, "{alive_color} {count}").unwrap();
                } else {
                    write!(out, "{default_color} 0").unwrap();
                }
            }
            writeln!(out).unwrap();
        }

        out.flush().unwrap();
    }
}