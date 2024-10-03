pub const GRID_WIDTH: usize = 50;
pub const GRID_HEIGHT: usize = 30;

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub cells: [[bool; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    pub fn new(initial_live_count: u8) -> Self {
        let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];
        let mut x = GRID_WIDTH / 2;
        let mut y = GRID_HEIGHT / 2;

        for i in 1..initial_live_count {
            loop {
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

                if grid[y][x] == false {
                    grid[y][x] = true;
                    break;
                }
            }
        }

        Grid { cells: grid }
    }

    pub fn next_generation(&mut self) -> usize {
        let mut updated_count: usize = 0;
        let mut new_cells = self.cells.to_owned();
        for (y, row) in self.cells.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                let count = self.count_living_neighbours(x, y);
                let mut new_state = alive;
                if *alive {
                    if count < 2 || count > 3 {
                        new_state = &false;
                    }
                } else if count == 3 {
                    new_state = &true;
                }

                if new_state != alive {
                    new_cells[y][x] = *new_state;
                    updated_count += 1;
                }
            }
        }

        self.cells = new_cells;
        updated_count
    }

    pub fn count_living_neighbours(&self, x: usize, y: usize) -> u8 {
        let is_on_left_edge = x == 0;
        let is_on_right_edge = x == GRID_WIDTH - 1;
        let is_on_top_edge = y == 0;
        let is_on_bottom_edge = y == GRID_HEIGHT - 1;
        let mut count = 0;

        if !is_on_left_edge {
            if self.is_alive(x - 1, y) {
                count += 1
            }
            if !is_on_top_edge && self.is_alive(x - 1, y - 1) {
                count += 1
            }
        }

        if !is_on_top_edge {
            if self.is_alive(x, y - 1) {
                count += 1
            }
            if !is_on_right_edge && self.is_alive(x + 1, y - 1) {
                count += 1
            }
        }

        if !is_on_right_edge {
            if self.is_alive(x + 1, y) {
                count += 1
            }
            if !is_on_bottom_edge && self.is_alive(x + 1, y + 1) {
                count += 1
            }
        }

        if !is_on_bottom_edge {
            if self.is_alive(x, y + 1) {
                count += 1
            }
            if !is_on_left_edge && self.is_alive(x - 1, y + 1) {
                count += 1
            }
        }

        count
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) -> bool {
        if x >= GRID_WIDTH || y > GRID_HEIGHT {
            false
        } else {
            self.cells[y][x] = !self.cells[y][x];
            true
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }
}
