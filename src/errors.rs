use std::fmt;

/**
 * Contains `game.rs` related errors.
 */
#[derive(Debug)]
pub enum GameError {
    /// When a value is **not** in the range `[0; Game.side_size]`.
    IllegalValue,
    /// When an existing cell prevents a value from being set.
    InvalidValue,
    /// When an asked position is out of the grid.
    IllegalPosition,
    /// When trying to set a value in a cell that already contains a non modifiable (initial) value.
    NonEmptyCell,
    /// Occurs when there is an error during the save file creation.
    CreateSaveFileError,
    /// Occurs when trying to save a game that does not have an attached file.
    NoSaveFile,
    /// Occurs when there is an error whilst writing the game save in a file.
    WriteSaveError,
    /// Occurs when unable to read a save file content.
    OpenFileError,
    /// Occurs when there is an error whilst parsing a save file.
    ParseSaveFileError,
    /// Occurs when the save file was loaded but contains erroneous values.
    IncorrectSaveFile,
    /// Occurs when unable to open an existing save file.
    OpenSaveFileError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::IllegalValue => write!(f, "Illegal value not in [0; side_size]."),
            GameError::InvalidValue => write!(f, "Invalid value for this cell."),
            GameError::IllegalPosition => write!(f, "This cell position is invalid."),
            GameError::NonEmptyCell => write!(f, "This cell already contain a value."),
            GameError::CreateSaveFileError => write!(f, "Unable to create the save file."),
            GameError::NoSaveFile => write!(
                f,
                "Cannot save the game because it does not have a save file attached."
            ),
            GameError::WriteSaveError => write!(f, "Unable to save to file."),
            GameError::OpenFileError => write!(f, "Unable to read the save file content."),
            GameError::ParseSaveFileError => write!(f, "Unable to parse the save file correctly."),
            GameError::IncorrectSaveFile => write!(f, "The save file contains erroneous data."),
            GameError::OpenSaveFileError => write!(f, "Unable to open the save file again."),
        }
    }
}

/**
 * Contains `solver.rs` related errors.
 */
pub enum SolverError {
    /// When a solver does not succeed in solving a game.
    FailedToSolve,
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolverError::FailedToSolve => write!(f, "Failed to solve the grid."),
        }
    }
}

/**
 * Contains errors related to the `Ui` trait of `ui.rs`.
 *
 * TODO: Implement the From::<GuiError> trait (or the other way around) so that the implementations
 * of Gui and Cli can return their own errors.
 */
#[derive(Debug)]
pub enum UiError {
    /// Occurs when there is a an error whilst loading the configuration file.
    LoadConfigError,
    /// Occurs when there is a syntax error in the configuration file.
    ConfigSyntaxError,
    /// Occurs when there is an error whilst loading the font file.
    LoadFontError,
    LoadSpriteError,
    /// Occurs when there is an error during the save file creation.
    CreateSaveFileError,
    /// Occurs when the loaded textures are missing a texture.
    MissingLoadedTexture,
    /// Occurs when a SDL2 error occurs.
    SDL2Error,
    /// occurs when there is an error writting the updated configuration file.
    WriteConfigError,
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // UiError::SaveError => write!(f, "Failed to save the game."),
            // UiError::FileWriteError => write!(f, "Failed to write to file."),
            UiError::LoadConfigError => write!(f, "Failed to load the configuration file."),
            UiError::ConfigSyntaxError => write!(f, "Configuration file syntax error."),
            UiError::LoadSpriteError => write!(f, "Failed to load the sprite."),
            UiError::LoadFontError => write!(f, "Unable to load font file."),
            UiError::CreateSaveFileError => write!(f, "Unable to create the save file."),
            UiError::SDL2Error => write!(f, "Generic SDL2 Error"),
            UiError::MissingLoadedTexture => write!(f, "Missing loaded texture."),
            UiError::WriteConfigError => write!(
                f,
                "An error occured when trying to write the updated configuration file."
            ),
        }
    }
}

impl From<GameError> for UiError {
    fn from(game_error: GameError) -> Self {
        match game_error {
            GameError::CreateSaveFileError => Self::CreateSaveFileError,
            _ => todo!(),
        }
    }
}
