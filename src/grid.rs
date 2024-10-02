pub const GRID_SIZE: usize = 20;

pub struct Grid {
    pub cells: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new(initial_live_count: u8) -> Self {
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
        let mut x = GRID_SIZE / 2;
        let mut y = GRID_SIZE / 2;

        for i in 1..initial_live_count {
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

        Grid { cells: grid }
    }

    pub fn next_generation(&mut self) -> usize {
        let mut updated_count: usize = 0;
        let mut new_cells = self.cells.to_owned();
        for (x, column) in self.cells.iter().enumerate() {
            for (y, alive) in column.iter().enumerate() {
                let count = self.get_living_neighbours_count(x, y);
                let mut new_state = alive;
                if *alive {
                    if count < 2 || count > 3 {
                        new_state = &false;
                    }
                } else if count == 3 {
                    new_state = &true;
                }

                if new_state != alive {
                    new_cells[x][y] = *new_state;
                    updated_count += 1;
                }
            }
        }

        self.cells = new_cells;
        updated_count
    }

    pub fn get_living_neighbours_count(&self, x: usize, y: usize) -> u8 {
        let is_on_left_edge = x == 0;
        let is_on_right_edge = x == GRID_SIZE - 1;
        let is_on_top_edge = y == 0;
        let is_on_bottom_edge = y == GRID_SIZE - 1;
        let mut count = 0;

        if !is_on_left_edge {
            if self.cells[x - 1][y] {
                count += 1
            }
            if !is_on_top_edge && self.cells[x - 1][y - 1] {
                count += 1
            }
        }

        if !is_on_top_edge {
            if self.cells[x][y - 1] {
                count += 1
            }
            if !is_on_right_edge && self.cells[x + 1][y - 1] {
                count += 1
            }
        }

        if !is_on_right_edge {
            if self.cells[x + 1][y] {
                count += 1
            }
            if !is_on_bottom_edge && self.cells[x + 1][y + 1] {
                count += 1
            }
        }

        if !is_on_bottom_edge {
            if self.cells[x][y + 1] {
                count += 1
            }
            if !is_on_left_edge && self.cells[x - 1][y + 1] {
                count += 1
            }
        }

        count
    }
}
