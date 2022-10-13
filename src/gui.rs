use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::render::WindowCanvas;
use sdl2::ttf::FontStyle;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

use core::time::Duration;
use std::fs;
use std::fs::File;
use std::io::prelude::Write;
use std::path::PathBuf;
use std::rc::Rc;

use crate::errors::UiError;
use crate::game::Game;
use crate::game_screen::GameScreen;
use crate::main_screen::MainScreen;
use crate::solver;
use crate::traits::{Displayable, GUIConfig, ScreenOutcome, Ui};

#[derive(Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Main,
    Game,
}

pub struct Gui<'a> {
    /// SDL2 window canvas
    canvas: WindowCanvas,
    /// SDL2 event pump
    event_pump: EventPump,
    /// Loaded SDL2 font pointer
    font: Rc<Font<'a, 'a>>,

    /// Looaded configuration file path
    config_path: PathBuf,
    /// Loaded config file
    config: GUIConfig,

    /// Currently displayed screen
    current_screen: Screen,

    /// Main screen instance
    main_screen: Option<MainScreen>,
    /// Game screen instance
    game_screen: Option<GameScreen<'a>>,
}

impl<'a> Gui<'a> {
    pub fn new(
        sdl_context: &'a Sdl,
        ttf_context: &'a Sdl2TtfContext,
        config_path: &String,
    ) -> Result<Self, UiError> {
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

        let config: GUIConfig = match serde_json::from_str(&config_txt) {
            Ok(config) => config,
            Err(e) => {
                eprintln!(
                    "Error while loading the configuration file {}: {}",
                    config_path, e
                );

                return Err(UiError::ConfigSyntaxError);
            }
        };

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Sudoku (Rust)", config.res_x as u32, config.res_y as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let mut font = match ttf_context.load_font(&config.font_path, 30) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadFontError);
            }
            Ok(font) => font,
        };
        font.set_style(FontStyle::BOLD);

        Ok(Gui {
            canvas,
            event_pump,
            font: Rc::new(font),

            config_path: PathBuf::from(config_path),
            config,

            current_screen: Screen::Main,
            main_screen: None,
            game_screen: None,
        })
    }

    pub fn init(&mut self) -> Result<(), UiError> {
        self.main_screen = Some(MainScreen::new());
        self.main_screen
            .as_mut()
            .unwrap()
            .init(&mut self.canvas, &self.config)?;
        self.game_screen = Some(GameScreen::new());
        self.game_screen
            .as_mut()
            .unwrap()
            .init(&mut self.canvas, &self.config)?;

        self.game_screen
            .as_mut()
            .unwrap()
            .set_font(self.font.clone());

        // If a game was loaded, set the boolean
        if self.game_screen.as_ref().unwrap().has_game() {
            self.main_screen.as_mut().unwrap().has_current_game = true;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), UiError> {
        // TODO: This does not solve the first black screen
        let mut outcome;
        self.main_screen.as_mut().unwrap().draw(&mut self.canvas)?;

        'running: loop {
            //if let Some(event) = self.event_pump.poll_event() {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        // If there is an ongoing game that isn't over, write its path in the
                        // configuration file as the game to resume in the next launch.
                        if self.current_screen == Screen::Game
                            && !self.game_screen.as_ref().unwrap().is_over()
                        {
                            self.config.game_resume_path = String::from(
                                self.game_screen
                                    .as_ref()
                                    .unwrap()
                                    .game
                                    .as_ref()
                                    .unwrap()
                                    .save_path
                                    .as_ref()
                                    .unwrap()
                                    .as_path()
                                    .to_str()
                                    .unwrap(),
                            );

                            if let Ok(mut file) = File::create(&self.config_path) {
                                if let Ok(config_txt) = serde_json::to_string_pretty(&self.config) {
                                    if file.write_all(config_txt.as_bytes()).is_err() {
                                        return Err(UiError::WriteConfigError);
                                    }
                                } else {
                                    return Err(UiError::WriteConfigError);
                                }
                            } else {
                                return Err(UiError::WriteConfigError);
                            }
                        }

                        break 'running;
                    }
                    Event::MouseButtonUp {
                        mouse_btn: MouseButton::Left,
                        ..
                    } => {
                        match self.current_screen {
                            Screen::Main => {
                                outcome = self.main_screen.as_mut().unwrap().update(&event)?;
                            }
                            Screen::Game => {
                                outcome = self.game_screen.as_mut().unwrap().update(&event)?;
                            }
                        };
                    }
                    Event::MouseMotion { .. } => match self.current_screen {
                        Screen::Main => {
                            outcome = self.main_screen.as_mut().unwrap().update(&event)?;
                        }
                        Screen::Game => {
                            outcome = ScreenOutcome::Unchanged;
                        }
                    },
                    _ => {
                        outcome = ScreenOutcome::Unchanged;
                    }
                }

                // println!("Event: [{:?}] -> Outcome: [{:?}]", event, outcome);

                match outcome {
                    ScreenOutcome::Updated => match self.current_screen {
                        Screen::Main => {
                            self.main_screen.as_mut().unwrap().draw(&mut self.canvas)?;
                        }
                        Screen::Game => {
                            self.game_screen.as_mut().unwrap().draw(&mut self.canvas)?;
                        }
                    },
                    ScreenOutcome::Resume => {
                        self.current_screen = Screen::Game;
                        self.game_screen.as_mut().unwrap().draw(&mut self.canvas)?;
                        continue 'running;
                    }
                    ScreenOutcome::NewGame => {
                        self.new_random_game()?;
                        self.current_screen = Screen::Game;
                        self.game_screen.as_mut().unwrap().draw(&mut self.canvas)?;
                        continue 'running;
                    }
                    ScreenOutcome::Exit => break 'running,

                    _ => {}
                }
            }

            // TODO: Why was this here ? How to count FPS ?
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}

impl<'a> Displayable for Gui<'a> {
    fn new() -> Self {
        todo!()
    }
    fn init(&mut self, _canvas: &mut Canvas<Window>, _config: &GUIConfig) -> Result<(), UiError> {
        todo!()
    }
    fn draw(&mut self, _canvas: &mut Canvas<Window>) -> Result<(), UiError> {
        todo!()
    }

    fn update(&mut self, _event: &Event) -> Result<ScreenOutcome, UiError> {
        todo!()
    }
}

impl Ui for Gui<'_> {
    fn new_random_game(&mut self) -> Result<(), UiError> {
        // Generate the game's saving path
        let current_utc = chrono::offset::Utc::now();
        let saving_path =
            format!("{}{}.game", self.config.save_folder_path, current_utc).replace(' ', " ");
        // Instanciate a new game with its saving path
        let mut new_game = Game::new(self.config.game_size, Some(&saving_path))?;
        new_game.clear();
        new_game.fill_rng(0);
        let solver = solver::Obvious;
        new_game.unfill(solver);
        new_game.save()?;
        // Attach the new game to the game screen
        self.game_screen.as_mut().unwrap().set_game(new_game);

        Ok(())
    }
}
