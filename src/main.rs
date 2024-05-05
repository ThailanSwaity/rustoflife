use crossterm::terminal;
use std::process;

use rustoflife::GameOfLife;

fn main() {
    let (cols, rows) = terminal::size().unwrap();

    if let Err(e) = rustoflife::run(GameOfLife::new(rows as usize, cols as usize)) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
