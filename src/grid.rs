pub const WIDTH: usize = 38;
pub const HEIGHT: usize = 30;

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub cells: [[bool; WIDTH]; HEIGHT],
}

pub enum Pattern {
    Random(usize),
    GosperGliderGun,
}

impl Grid {
    pub fn new(pattern: Pattern) -> Self {
        Grid {
            cells: generate_pattern(pattern),
        }
    }

    pub fn reset(&mut self, pattern: Pattern) {
        self.cells = generate_pattern(pattern);
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
        let is_on_right_edge = x == WIDTH - 1;
        let is_on_top_edge = y == 0;
        let is_on_bottom_edge = y == HEIGHT - 1;
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
        if x >= WIDTH || y > HEIGHT {
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

fn generate_pattern(pattern: Pattern) -> [[bool; WIDTH]; HEIGHT] {
    match pattern {
        Pattern::Random(alive_count) => generate_random(alive_count),
        Pattern::GosperGliderGun => generate_gosper_glider_gun(),
    }
}

fn generate_random(alive_count: usize) -> [[bool; WIDTH]; HEIGHT] {
    let mut grid = [[false; WIDTH]; HEIGHT];
    let mut x = WIDTH / 2;
    let mut y = HEIGHT / 2;

    for i in 1..alive_count {
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

    grid
}

fn generate_gosper_glider_gun() -> [[bool; WIDTH]; HEIGHT] {
    let mut grid = [[false; WIDTH]; HEIGHT];
    add_block(&mut grid, 1, 7);
    add_gosper_glider_gun_1(&mut grid, 11, 5);
    add_gosper_glider_gun_2(&mut grid, WIDTH - 17, 3);
    add_block(&mut grid, WIDTH - 3, 5);
    grid
}

fn add_block(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y + 0][x + 0] = true;
    grid[y + 0][x + 1] = true;
    grid[y + 1][x + 0] = true;
    grid[y + 1][x + 1] = true;
}

fn add_gosper_glider_gun_1(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y + 0][x + 2] = true;
    grid[y + 0][x + 3] = true;
    grid[y + 1][x + 1] = true;
    grid[y + 1][x + 5] = true;
    grid[y + 2][x + 0] = true;
    grid[y + 2][x + 6] = true;
    grid[y + 3][x + 0] = true;
    grid[y + 3][x + 4] = true;
    grid[y + 3][x + 6] = true;
    grid[y + 3][x + 7] = true;
    grid[y + 4][x + 0] = true;
    grid[y + 4][x + 6] = true;
    grid[y + 5][x + 1] = true;
    grid[y + 5][x + 5] = true;
    grid[y + 6][x + 2] = true;
    grid[y + 6][x + 3] = true;
}

fn add_gosper_glider_gun_2(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y + 0][x + 4] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 1][x + 4] = true;
    grid[y + 2][x + 0] = true;
    grid[y + 2][x + 1] = true;
    grid[y + 3][x + 0] = true;
    grid[y + 3][x + 1] = true;
    grid[y + 4][x + 0] = true;
    grid[y + 4][x + 1] = true;
    grid[y + 5][x + 2] = true;
    grid[y + 5][x + 4] = true;
    grid[y + 6][x + 4] = true;
}
