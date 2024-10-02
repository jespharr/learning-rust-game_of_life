use std::io::{stdout, Write};
use termion::{color, cursor, screen::IntoAlternateScreen};

const GRID_SIZE: usize = 20;

fn main() {
    let update_frequency = std::time::Duration::from_secs(1);
    let mut grid = initialize_grid();
    let mut out = stdout().into_alternate_screen().unwrap();
    writeln!(out, "{}", cursor::Hide).unwrap();

    loop {
        grid = update_grid(&grid);
        draw_grid(&grid, &mut out);
        std::thread::sleep(update_frequency);
    }
}

fn initialize_grid() -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    let mut x = GRID_SIZE / 2;
    let mut y = GRID_SIZE / 2;

    for i in 0..4 {
        if i % 2 == 0 {
            if rand::random() {
                x += 1;
            } else {
                x -= 1;
            }
        } else {
            if rand::random() {
                y += 1;
            } else {
                y -= 1;
            }
        }

        grid[x][y] = true;
    }

    grid
}

fn update_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE]) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut new_grid = grid.to_owned();
    for (x, column) in grid.iter().enumerate() {
        for (y, alive) in column.iter().enumerate() {
            let count = count_living_neighbours(grid, x, y);
            let mut new_state = alive;
            if *alive {
                if count < 2 || count > 3 {
                    new_state = &false;
                }
            } else if count == 3 {
                new_state = &true;
            }
            new_grid[x][y] = *new_state;
        }
    }

    new_grid
}

fn count_living_neighbours(grid: &[[bool; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) -> u8 {
    let is_on_left_edge = x == 0;
    let is_on_right_edge = x == GRID_SIZE - 1;
    let is_on_top_edge = y == 0;
    let is_on_bottom_edge = y == GRID_SIZE - 1;
    let mut count = 0;

    if !is_on_left_edge {
        if grid[x - 1][y] {
            count += 1
        }
        if !is_on_top_edge && grid[x - 1][y - 1] {
            count += 1
        }
    }

    if !is_on_top_edge {
        if grid[x][y - 1] {
            count += 1
        }
        if !is_on_right_edge && grid[x + 1][y - 1] {
            count += 1
        }
    }

    if !is_on_right_edge {
        if grid[x + 1][y] {
            count += 1
        }
        if !is_on_bottom_edge && grid[x + 1][y + 1] {
            count += 1
        }
    }

    if !is_on_bottom_edge {
        if grid[x][y + 1] {
            count += 1
        }
        if !is_on_left_edge && grid[x - 1][y + 1] {
            count += 1
        }
    }

    count
}

fn draw_grid(grid: &[[bool; GRID_SIZE]; GRID_SIZE], out: &mut dyn Write) {
    let default_color = color::Fg(color::Reset);
    let alive_color = color::Fg(color::Cyan);

    writeln!(out, "{}", cursor::Goto(1, 1)).unwrap();

    for (x, row) in grid.iter().enumerate() {
        for (y, alive) in row.iter().enumerate() {
            if *alive {
                let count = count_living_neighbours(grid, x, y);
                write!(out, "{alive_color} {count}").unwrap();
            } else {
                write!(out, "{default_color} 0").unwrap();
            }
        }
        writeln!(out).unwrap();
    }

    out.flush().unwrap();
}
