use crate::errors::UiError;
use crate::game::Game;
use crate::solver;
use crate::traits::{CliConfig, Ui};

use colored::*;
use core::str::FromStr;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::Write;

#[allow(dead_code)]
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

/**
 * This represents a Command Line Interface (CLI) for the user to play with.
 */
pub struct Cli {
    /// Loaded configuration file
    _config: CliConfig,
    /// The game instance currently being played.
    game: Game,
    /// The current value that ought to be highlighted when printing the grid.
    highlighted_value: Option<u8>,
}

impl Cli {
    pub fn new(config_path: &str) -> Result<Self, UiError> {
        // Config loader.
        let config_txt = match fs::read_to_string(config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Error while loading the configuration file {}: {}",
                    config_path, e
                );

                return Err(UiError::LoadConfigError);
            }
        };

        let config: CliConfig = match serde_json::from_str(&config_txt) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Error while loading the configuration file {}: {}",
                    config_path, e
                );

                return Err(UiError::ConfigSyntaxError);
            }
        };

        // Get the save folder path
        // Generate the game's saving path
        let current_utc = chrono::offset::Utc::now();
        let saving_path = format!("{}{}.game", config.save_folder_path, current_utc);

        // Instanciate a game from its size
        let game = Game::new(config.game_size, Some(&saving_path))?;
        // Instanciate Self.
        Ok(Cli {
            game,
            _config: config,

            highlighted_value: None,
        })
    }

    /**
     * This function is a helper to ask the user for a number that is within the given range.
     */
    pub fn ask_number<Type>(range: std::ops::RangeInclusive<Type>, prompt: Option<&str>) -> Type
    where
        Type: PartialOrd + FromStr + Default,
    {
        // Initialise the return value with 'the zero' of the asked Type.
        let mut ret: Type = Default::default();

        // Loop until we have a value that is within the asked range.
        while !range.contains(&ret) {
            // Force print a prompt and get the user input
            print!("{}", prompt.unwrap_or("Your input: "));
            io::stdout().flush().unwrap();
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            // Parse the user input into a value of the asked Type.
            let trimmed = input_text.trim();
            ret = trimmed.parse::<Type>().unwrap_or_default();
        }

        // Return
        ret
    }

    /**
     * Main game loop.
     *
     */
    pub fn run(&mut self) -> Result<(), UiError> {
        self.new_random_game().unwrap();
        while !self.game.is_done() {
            // Reset the screen
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            // self.highlighted_value = Some(3);
            // Print the grid
            println!("{}", self);

            // Ask for move: row, column and value
            println!("Your move:");
            let row = Self::ask_number::<usize>(1..=self.game.side_size, Some("Row: "));
            let column = Self::ask_number::<usize>(1..=self.game.side_size, Some("Column: "));
            let value = Self::ask_number::<u8>(1..=(self.game.side_size as u8), Some("Value: "));

            // Do the move if it is valid, otherwise display why it is not.
            match self.game.do_move(row - 1, column - 1, value) {
                Ok(_) => continue,
                Err(e) => {
                    println!("{}", e);
                    pause();
                }
            };
        }

        Ok(())
    }
}

impl fmt::Display for Cli {
    /**
     * Displays the grid.
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Game:\n")?;

        // For each row
        for i in 0..self.game.side_size {
            // Print the horizontal line in color if its zero modulo the grid size.
            if (i % self.game.size) == 0 {
                writeln!(
                    f,
                    "{}{}",
                    "+---".repeat(self.game.side_size).bright_blue().bold(),
                    "+".bright_blue().bold()
                )?;
            }
            // Else, print it in normal color
            else {
                writeln!(
                    f,
                    "{}{}",
                    format!(
                        "{}---{}",
                        "+".bright_blue().bold(),
                        "+---".repeat(self.game.size - 1)
                    )
                    .repeat(self.game.size),
                    "+".bright_blue().bold()
                )?;
            }

            // Printing the number row
            for j in 0..self.game.side_size {
                // Print the separator in color if its index is equal to zero modulo the grid size.
                if (j % self.game.size) == 0 {
                    write!(f, "{} ", "|".bright_blue().bold())?;
                }
                // Else, print it in normal color
                else {
                    write!(f, "| ")?;
                }

                // Get the cell's value as a string or a space if it's zero.
                let value = self.game.grid[self.game.index(i, j)].value;
                let mut value_string = match value {
                    0 => " ".to_string(),
                    _ => value.to_string(),
                };

                // If the value is the currently highlighted one, highlight it.
                if self.highlighted_value.unwrap_or(0) == value {
                    value_string = value_string.bright_red().to_string();
                }

                // Print the cell's value
                write!(f, "{} ", value_string)?;
            }
            // Print the last separator of the row in color.
            writeln!(f, "{}", "|".bright_blue().bold())?;
        }

        // Write the last horizontal line in color.
        writeln!(
            f,
            "{}{}",
            "+---".repeat(self.game.side_size).bright_blue().bold(),
            "+".bright_blue().bold()
        )?;

        Ok(())
    }
}

impl Ui for Cli {
    /**
     * This function generates a CLI instance from a JSON configuration file.
     */

    /**
     * This function initialises the `self.game` instance with a new random solvable game.
     */
    fn new_random_game(&mut self) -> Result<(), UiError> {
        self.game.clear();
        self.game.fill_rng(0);
        let solver = solver::Obvious;
        self.game.unfill(solver);

        Ok(())
    }
}
