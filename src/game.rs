use crate::errors::GameError;
use crate::solver::Solver;

use colored::*;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, Write};
use std::path::PathBuf;

const MAX_UNFILL_ATTEMPTS: usize = 3;
const MIN_CLUES: usize = 17;

lazy_static! {
    static ref RE_GAME_SIZE: regex::Regex = Regex::new(r"(?m)^game_size: ([345])$").unwrap();
    static ref RE_SELECTED: regex::Regex = Regex::new(r"(?m)^selected: (\d+)$").unwrap();
    static ref RE_CELLS: regex::Regex = Regex::new(r"(?m)^cells: (\d/[IN],?)+$?").unwrap();
    static ref RE_CELL: regex::Regex = Regex::new(r"(\d)/([IN]),?").unwrap();
}

#[derive(Clone)]
pub struct Cell {
    pub value: u8,
    pub initial: bool,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<u8> for Cell {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

impl PartialEq<u8> for &Cell {
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

// const COLOR: <(dyn colored::Colorize + 'static)>::Fn = colored::Colorize::blue;
// const COLOR: fn(String) -> ColoredString = colored::Colorize::blue;
// const COLOR: <(dyn colored::Colorize + 'static) as Trait>::Fn = colored::Colorize::blue;
//
pub struct Game {
    pub size: usize,
    /// Square side size of the grid.
    pub side_size: usize,
    /// The file this game ought to be saved in.
    save_file: Option<File>,
    /// The path of the save file of this game.
    pub save_path: Option<PathBuf>,
    /// The currently selected cell's index
    pub selected_index: Option<usize>,
    pub selected_value: Option<u8>,
    /// The actual grid.
    pub grid: Vec<Cell>,
}

impl Game {
    pub fn new(size: usize, saving_path: Option<&str>) -> Result<Self, GameError> {
        let side_size = size * size;
        let (save_path, save_file) = match saving_path {
            Some(path) => match File::create(path) {
                Ok(file_handle) => (Some(PathBuf::from(saving_path.unwrap())), Some(file_handle)),
                Err(_) => {
                    dbg!(path);
                    return Err(GameError::CreateSaveFileError);
                }
            },
            None => (None, None),
        };

        Ok(Game {
            size,
            side_size,
            selected_index: None,
            selected_value: None,
            save_file,
            save_path,
            grid: vec![
                Cell {
                    value: 0,
                    initial: false
                };
                side_size * side_size
            ],
        })
    }

    pub fn from_file(path: &str) -> Result<Self, GameError> {
        let file_content = match fs::read_to_string(path) {
            Ok(fc) => fc,
            Err(_) => return Err(GameError::OpenFileError),
        };

        let game_size = match RE_GAME_SIZE.captures(&file_content) {
            Some(m) => {
                let parsed = match m.get(1).unwrap().as_str().parse::<usize>() {
                    Ok(p) => p,
                    Err(_) => return Err(GameError::ParseSaveFileError),
                };
                parsed
            }
            None => return Err(GameError::ParseSaveFileError),
        };

        let selected_index = match RE_SELECTED.captures(&file_content) {
            Some(m) => {
                let parsed = match m.get(1).unwrap().as_str().parse::<usize>() {
                    Ok(p) => p,
                    Err(_) => return Err(GameError::ParseSaveFileError),
                };

                Some(parsed)
            }
            None => None,
        };

        let cells = match RE_CELLS.captures(&file_content) {
            Some(m) => {
                let mut cells = Vec::new();
                for mat in RE_CELL.captures_iter(m.get(0).unwrap().as_str()) {
                    let value = match mat.get(1).unwrap().as_str().parse::<u8>() {
                        Ok(v) => v,
                        Err(_) => return Err(GameError::ParseSaveFileError),
                    };

                    let initial = match mat.get(2).unwrap().as_str() {
                        "I" => true,
                        "N" => false,
                        _ => return Err(GameError::ParseSaveFileError),
                    };

                    cells.push(Cell { value, initial });
                }

                cells
            }
            None => return Err(GameError::ParseSaveFileError),
        };

        // The number of cells is the game's size squared
        let side_size = game_size * game_size;
        // Double check that we loaded exactly the good number of cells
        if cells.len() != (side_size * side_size) {
            println!("{} != {}", cells.len(), side_size);
            return Err(GameError::IncorrectSaveFile);
        }

        // Double check that if we have a selected cell, its index is valid
        if selected_index.is_some() && selected_index.unwrap() >= (side_size * side_size) {
            println!("Invalid selected: {}", selected_index.unwrap());
            return Err(GameError::IncorrectSaveFile);
        }

        let selected_value = match selected_index.is_some() && cells[selected_index.unwrap()] != 0 {
            true => Some(cells[selected_index.unwrap()].value),
            false => None,
        };

        // Finally open the save file in order to continue saving in it
        let file_handle = match OpenOptions::new().read(true).write(true).open(path) {
            Ok(fd) => Some(fd),
            Err(_) => return Err(GameError::OpenSaveFileError),
        };

        Ok(Game {
            size: game_size,
            side_size,
            save_path: Some(PathBuf::from(path)),
            save_file: file_handle,
            selected_index,
            selected_value,
            grid: cells,
        })
    }

    /// Resets the grid with all zeros.
    pub fn clear(&mut self) {
        self.grid = vec![
            Cell {
                value: 0,
                initial: false
            };
            self.side_size * self.side_size
        ];
    }

    /// Counts the number of **empty** boxes in the grid.
    #[cfg(test)]
    pub fn nb_empty(&self) -> usize {
        return self.grid.iter().filter(|&x| *x == 0u8).count();
    }

    /// Returns an iterator over the empty cells of the grid.
    pub fn _empties(&self) -> impl Iterator<Item = usize> + '_ {
        (0..(self.side_size * self.side_size)).filter(|i| self.grid[*i].value == 0)
    }

    /// Counts the number of **non empty** boxes in the grid.
    pub fn nb_non_empty(&self) -> usize {
        return self.grid.iter().filter(|&x| *x != 0u8).count();
    }

    /// Returns the coordinates of a given index in the grid, as (row, column).
    pub fn coordinates(&self, index: usize) -> (usize, usize) {
        (index / self.side_size, index % self.side_size)
    }

    /// Function that gets the index of the grid's box located at row `r` and column `c`.
    pub fn index(&self, r: usize, c: usize) -> usize {
        r * self.side_size + c
    }

    /// Function that gets the `self.side_size` elements that are in the column `c`.
    pub fn column(&self, c: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.side_size).map(move |x| self.index(x, c))
    }

    /// Function that gets the `self.side_size` elements that are in the row `r`.
    pub fn row(&self, r: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.side_size).map(move |x| self.index(r, x))
    }

    /// Function that gets the `self.side_size` elements that are in the group of the
    /// grid's value located in row `r` and column `c`.
    pub fn group(&self, r: usize, c: usize) -> impl Iterator<Item = usize> + '_ {
        let start_row = (r / self.size) * self.size;
        let start_col = (c / self.size) * self.size;

        (start_row..start_row + self.size)
            .flat_map(move |r| (start_col..start_col + self.size).map(move |c| self.index(r, c)))
    }

    /// Returns the concatenation of `row()`, `column()` and `group()` functions.
    pub fn neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = usize> + '_ {
        self.column(c).chain(self.row(r)).chain(self.group(r, c))
    }

    /// Returns the values that are not taken by any neighbor.
    pub fn valids(&self, index: usize) -> Vec<u8> {
        let (r, c) = self.coordinates(index);
        let mut possibles: HashSet<u8> = (1..=self.side_size as u8).collect();
        let used: Vec<u8> = self.neighbors(r, c).map(|i| self.grid[i].value).collect();

        for value in used {
            possibles.remove(&value);

            // Stop if theren are possible values
            if possibles.is_empty() {
                break;
            }
        }

        possibles.into_iter().collect()
    }

    /// Checks if the grid is correctly completed.
    /// Returns `true` if yes, `false` otherwise.
    pub fn is_done(&self) -> bool {
        // Check that all the grid is filled.
        if self.grid.iter().any(|x| x.value == 0) {
            return false;
        }

        // Check row
        for row in 0..self.side_size {
            if (1..=self.side_size).any(|v| {
                !self
                    .row(row)
                    .map(|i| self.grid[i].value)
                    .any(|x| x == (v as u8))
            }) {
                return false;
            }
        }

        // Check columns
        for col in 0..self.side_size {
            if (1..=self.side_size).any(|v| {
                !self
                    .column(col)
                    .map(|i| self.grid[i].value)
                    .any(|x| x == (v as u8))
            }) {
                return false;
            }
        }

        // Check groups
        for group_x in 0..self.size {
            for group_y in 0..self.size {
                if (1..=self.side_size).any(|v| {
                    !self
                        .group(group_x * self.size, group_y * self.size)
                        .map(|i| self.grid[i].value)
                        .any(|x| x == (v as u8))
                }) {
                    return false;
                }
            }
        }

        true
    }

    pub fn do_move(&mut self, r: usize, c: usize, value: u8) -> Result<(), GameError> {
        // Check the position is legal
        if r >= self.side_size || c >= self.side_size {
            return Err(GameError::IllegalPosition);
        }

        // Check the new value is legal
        if value == 0 || value > self.side_size as u8 {
            return Err(GameError::IllegalValue);
        }

        // Get the index of the target box
        let index = self.index(r, c);

        // If the box already contains a value
        if self.grid[index] != 0 {
            return Err(GameError::NonEmptyCell);
        }

        // Check the new value is valid
        if !self.valids(index).contains(&value) {
            return Err(GameError::InvalidValue);
        }

        // Set the new value
        self.grid[index] = Cell {
            value,
            initial: false,
        };

        // If this game is attached to a save file, save the game after doing the move
        if self.save_file.is_some() {
            self.save()?;
        }

        Ok(())
    }

    pub fn fill_rng(&mut self, current_cell: usize) -> bool {
        if current_cell >= self.side_size * self.side_size {
            return true;
        }
        let v = self.valids(current_cell);

        for n in v {
            self.grid[current_cell] = Cell {
                value: n,
                initial: true,
            };

            if self.fill_rng(current_cell + 1) {
                return true;
            }
        }

        self.grid[current_cell] = Cell {
            value: 0,
            initial: false,
        };
        false
    }

    /**
     * This function unfills the grid as long as the given `solder` can solve it.
     *
     * Note: It also uses the constant `MIN_CLUES` and will leave at least `MIN_CLUES`
     * values set in the grid.
     *
     * Note: It also will do at most `MAX_UNFILL_ATTEMPT` at unfilling until there are `MIN_CLUES`
     * values set left.
     */
    pub fn unfill<S: Solver>(&mut self, solver: S) {
        // TODO: Use difficulty to define ranges of number of clues that'll represent
        // the difficulty of the grid.
        // Example:
        //  - [17;30] = Very hard
        //  ......
        //  - [45;55] = Easy
        //

        // Attempt counter and random number generator
        let (mut attempt, mut rng) = (MAX_UNFILL_ATTEMPTS, rand::thread_rng());

        // As long as we have attempts left and more than the minimum clues set in the grid
        while attempt > 0 && self.nb_non_empty() >= MIN_CLUES {
            // Find a random non empty box
            let mut random_index = rng.gen_range(0..(self.side_size * self.side_size));
            while self.grid[random_index] == 0 {
                random_index = rng.gen_range(0..(self.side_size * self.side_size));
            }

            // Keep a track of the old value of the random box and empty it
            let old_value = self.grid[random_index].value;
            self.grid[random_index] = Cell {
                value: 0,
                initial: false,
            };
            // Make a copy of new modified game
            let mut game_copy = Game {
                size: self.size,
                side_size: self.side_size,
                selected_index: None,
                selected_value: None,
                save_path: None,
                save_file: None,
                grid: self.grid.clone(),
            };

            // Check if we can still solve the grid, if not reverse the change (emptying a
            // random box) and decrement the number of attempts left
            if solver.solve(&mut game_copy).is_err() {
                self.grid[random_index] = Cell {
                    value: old_value,
                    initial: true,
                };
                attempt -= 1;
            }
        }
    }

    pub fn save(&mut self) -> Result<(), GameError> {
        // Rewind the file
        if self.save_file.is_none() {
            return Err(GameError::NoSaveFile);
        }

        match self.save_file.as_ref().unwrap().rewind() {
            Ok(_) => (),
            Err(_) => return Err(GameError::WriteSaveError),
        }

        // Write the game size on the first line
        match writeln!(
            &mut self.save_file.as_ref().unwrap(),
            "game_size: {}",
            self.size
        ) {
            Ok(_) => (),
            Err(_) => return Err(GameError::WriteSaveError),
        }

        // Then if any, write the currently selected cell
        if self.selected_index.is_some() {
            match writeln!(
                &mut self.save_file.as_ref().unwrap(),
                "selected: {}",
                self.selected_index.unwrap()
            ) {
                Ok(_) => (),
                Err(_) => return Err(GameError::WriteSaveError),
            }
        }

        // Generate a string containing comma-separated values and "I" for initial "N" for non
        // initial value.
        let values = self
            .grid
            .iter()
            .map(|x| {
                let c = match x.initial {
                    true => "I",
                    false => "N",
                };
                format!("{}/{}", x.value, c)
            })
            .collect::<Vec<String>>()
            .join(",");

        // Write the grid's values to the file
        match writeln!(&mut self.save_file.as_ref().unwrap(), "cells: {}", values) {
            Ok(_) => (),
            Err(_) => return Err(GameError::WriteSaveError),
        }

        Ok(())
    }
}

/**
 * Implementation of the `fmt::Display` trait for a game.
 *
 * This will display the grid and if wanted highlight numbers.
 */
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Game:\n")?;

        for i in 0..self.side_size {
            if (i % self.size) == 0 {
                writeln!(
                    f,
                    "{}{}",
                    "+---".repeat(self.side_size).blue().bold(),
                    "+".blue().bold()
                )?;
            } else {
                writeln!(
                    f,
                    "{}{}",
                    format!("{}---{}", "+".blue().bold(), "+---".repeat(self.size - 1))
                        .repeat(self.size),
                    "+".blue().bold()
                )?;
            }

            for j in 0..self.side_size {
                if (j % self.size) == 0 {
                    write!(f, "{} ", "|".blue().bold())?;
                } else {
                    write!(f, "| ")?;
                }

                let val = match self.grid[self.index(i, j)].value {
                    0 => " ".to_string(),
                    _ => self.grid[self.index(i, j)].value.to_string(),
                };

                write!(f, "{} ", val)?;
            }
            writeln!(f, "{}", "|".blue().bold())?;
        }

        writeln!(
            f,
            "{}{}",
            "+---".repeat(self.side_size).blue().bold(),
            "+".blue().bold()
        )?;

        Ok(())
    }
}
