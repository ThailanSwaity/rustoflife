use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};
use std::io::{self, Write};

use rand::Rng;

struct GameOfLife {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    fn new(rows: usize, cols: usize) -> Self {
        GameOfLife {
            rows,
            cols,
            grid: vec![vec![false; rows]; cols],
        }
    }

    fn step(&mut self) {
        todo!("simulation stepping");
    }

    fn is_extinct(&self) -> bool {
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

fn main() {
    // terminal::enable_raw_mode().unwrap();
    let (cols, rows) = terminal::size().unwrap();

    let mut stdout = io::stdout();

    let mut game_grid = GameOfLife::new(rows as usize, cols as usize);

    for x in 0..game_grid.cols {
        for y in 0..game_grid.rows {
            game_grid.grid[x][y] = rand::thread_rng().gen_bool(0.5);
        }
    }

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    // Run until the last cell dies
    while !game_grid.is_extinct() {
        game_grid.step();

        // Draws
        for x in 0..game_grid.cols {
            for y in 0..game_grid.rows {
                if !game_grid.grid[x][y] {
                    stdout
                        .queue(cursor::MoveTo(x as u16, y as u16))
                        .unwrap()
                        .queue(style::Print("#"))
                        .unwrap();
                }
            }
        }
        stdout.flush().unwrap();
    }

    // terminal::disable_raw_mode().unwrap();
}
