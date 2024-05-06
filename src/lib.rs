use crossterm::{cursor, style, terminal, QueueableCommand};
use rand::Rng;
use std::error::Error;
use std::io::{self, Write};
use std::{thread, time};

pub struct GameOfLife {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Self {
        GameOfLife {
            rows,
            cols,
            grid: vec![vec![false; rows]; cols],
        }
    }

    pub fn tick(&mut self) {
        let mut t_grid: Vec<Vec<bool>> = vec![vec![false; self.rows]; self.cols];

        for x in 0..self.cols {
            for y in 0..self.rows {
                let neighbours = self.get_neighbours(x as u16, y as u16);

                if self.grid[x][y] && neighbours < 2 {
                    t_grid[x][y] = false;
                } else if self.grid[x][y] && (neighbours == 2 || neighbours == 3) {
                    t_grid[x][y] = true;
                } else if self.grid[x][y] && neighbours > 3 {
                    t_grid[x][y] = false;
                } else if !self.grid[x][y] && neighbours == 3 {
                    t_grid[x][y] = true;
                }
            }
        }

        self.grid = t_grid;
    }

    fn get_neighbours(&self, x: u16, y: u16) -> u16 {
        // TODO: Make this wrap around
        let mut neighbours = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let ax: i16 = x as i16 + dx;
                let ay: i16 = y as i16 + dy;

                let ax = (ax).rem_euclid(self.cols as i16);
                let ay = (ay).rem_euclid(self.rows as i16);

                if self.grid[ax as usize][ay as usize] {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    pub fn is_extinct(&self) -> bool {
        for x in 0..self.cols {
            for y in 0..self.rows {
                if self.grid[x][y] {
                    return false;
                }
            }
        }
        true
    }
}

pub fn run(mut game_grid: GameOfLife) -> Result<(), Box<dyn Error>> {
    for x in 0..game_grid.cols {
        for y in 0..game_grid.rows {
            game_grid.grid[x][y] = rand::thread_rng().gen_bool(0.2);
        }
    }

    let mut stdout = io::stdout();
    // Run until the last cell dies
    while !game_grid.is_extinct() {
        game_grid.tick();

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        // Draws
        for x in 0..game_grid.cols {
            for y in 0..game_grid.rows {
                if game_grid.grid[x][y] {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))?
                        .queue(style::Print("█"))?;
                }
            }
        }
        stdout.flush()?;
        thread::sleep(time::Duration::from_millis(250));
    }
    Ok(())
}
