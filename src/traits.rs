use crate::errors::UiError;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CliConfig {
    /// Games save folder
    pub save_folder_path: String,

    /// Game to resume
    pub game_resume_path: String,

    /// Game size
    pub game_size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GUIConfig {
    /// Path of the games-save folder.
    pub save_folder_path: String,

    /// Path of the game to resume.
    pub game_resume_path: String,

    /// Size of the board.
    pub game_size: usize,

    /// Horizontal resolution of the game window.
    pub res_x: usize,
    /// Vertical resolution of the game window.
    pub res_y: usize,

    /// Path of the font used to draw the game board.
    pub font_path: String,

    /// Buttons images paths
    pub btn_resume_path: String,
    pub btn_new_game_path: String,
    pub btn_exit_path: String,
    pub btn_resume_hover_path: String,
    pub btn_new_game_hover_path: String,
    pub btn_exit_hover_path: String,
}

pub trait Ui {
    fn new_random_game(&mut self) -> Result<(), UiError>;
}

#[derive(Debug)]
pub enum ScreenOutcome {
    Unchanged,
    Updated,
    Resume,
    NewGame,
    Exit,
}

pub trait Displayable {
    fn new() -> Self;
    fn init(&mut self, canvas: &mut Canvas<Window>, config: &GUIConfig) -> Result<(), UiError>;
    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), UiError>;
    fn update(&mut self, event: &Event) -> Result<ScreenOutcome, UiError>;
}
