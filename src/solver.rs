use crate::errors;
use crate::game::{Cell, Game};

pub trait Solver {
    fn solve(&self, game: &mut Game) -> Result<(), errors::SolverError>;
}

pub struct Obvious;

impl Solver for Obvious {
    fn solve(&self, game: &mut Game) -> Result<(), errors::SolverError> {
        if game.is_done() {
            return Ok(());
        }

        for i in 0..(game.side_size * game.side_size) {
            if game.grid[i] == 0 && game.valids(i).len() == 1 {
                game.grid[i] = Cell {
                    value: game.valids(i)[0],
                    initial: false,
                };
                return self.solve(game);
            }
        }

        Err(errors::SolverError::FailedToSolve)
    }
}
