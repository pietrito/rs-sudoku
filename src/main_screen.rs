use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::collections::HashMap;

use crate::errors::UiError;
use crate::traits::{Displayable, GUIConfig, ScreenOutcome};

pub struct MainScreen {
    // Textures
    textures: HashMap<String, (Texture, Rect)>,

    // Current Textures
    current_btn_resume: String,
    current_btn_new_game: String,
    current_btn_exit: String,

    // Outside vars
    pub has_current_game: bool,
}

impl Displayable for MainScreen {
    fn new() -> Self {
        // Texture creator associated to the current canvas

        MainScreen {
            textures: HashMap::new(),

            current_btn_resume: "btn_resume".to_string(),
            current_btn_new_game: "btn_new_game".to_string(),
            current_btn_exit: "btn_exit".to_string(),

            has_current_game: false,
        }
    }

    fn init(&mut self, canvas: &mut Canvas<Window>, config: &GUIConfig) -> Result<(), UiError> {
        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        // 'RESUME' button
        let btn_resume = match Surface::from_file(&config.btn_resume_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_resume_tex = texture_creator
            .create_texture_from_surface(&btn_resume)
            .unwrap();

        // 'RESUME HOVER' button
        let btn_resume_hover = match Surface::from_file(&config.btn_resume_hover_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_resume_hover_tex = texture_creator
            .create_texture_from_surface(&btn_resume_hover)
            .unwrap();

        // 'NEW GAME' button
        let btn_new_game = match Surface::from_file(&config.btn_new_game_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_new_game_tex = texture_creator
            .create_texture_from_surface(&btn_new_game)
            .unwrap();

        // 'NEW GAME HOVER' button
        let btn_new_game_hover = match Surface::from_file(&config.btn_new_game_hover_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_new_game_hover_tex = texture_creator
            .create_texture_from_surface(&btn_new_game_hover)
            .unwrap();

        // 'EXIT' button
        let btn_exit = match Surface::from_file(&config.btn_exit_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_exit_tex = texture_creator
            .create_texture_from_surface(&btn_exit)
            .unwrap();

        // 'EXIT HOVER' button
        let btn_exit_hover = match Surface::from_file(&config.btn_exit_hover_path) {
            Err(e) => {
                eprintln!("{}", e);
                return Err(UiError::LoadSpriteError);
            }
            Ok(surface) => surface,
        };
        let btn_exit_hover_tex = texture_creator
            .create_texture_from_surface(&btn_exit_hover)
            .unwrap();

        let btn_resume_pos = Rect::from_center(
            Point::new(
                (canvas.viewport().width() / 2).try_into().unwrap(),
                (canvas.viewport().height() / 2
                    - btn_new_game_tex.query().height / 2
                    - 10
                    - btn_resume_tex.query().height / 2)
                    .try_into()
                    .unwrap(),
            ),
            btn_resume_tex.query().width,
            btn_resume_tex.query().height,
        );

        let btn_new_game_pos = Rect::from_center(
            Point::new(
                (canvas.viewport().width() / 2).try_into().unwrap(),
                (canvas.viewport().height() / 2).try_into().unwrap(),
            ),
            btn_new_game_tex.query().width,
            btn_new_game_tex.query().height,
        );
        let btn_exit_pos = Rect::from_center(
            Point::new(
                (canvas.viewport().width() / 2).try_into().unwrap(),
                (canvas.viewport().height() / 2
                    + btn_new_game_tex.query().height / 2
                    + 10
                    + btn_exit_tex.query().height / 2)
                    .try_into()
                    .unwrap(),
            ),
            btn_exit_tex.query().width,
            btn_exit_tex.query().height,
        );

        self.textures
            .insert(String::from("btn_resume"), (btn_resume_tex, btn_resume_pos));
        self.textures.insert(
            String::from("btn_resume_hover"),
            (btn_resume_hover_tex, btn_resume_pos),
        );

        self.textures.insert(
            String::from("btn_new_game"),
            (btn_new_game_tex, btn_new_game_pos),
        );
        self.textures.insert(
            String::from("btn_new_game_hover"),
            (btn_new_game_hover_tex, btn_new_game_pos),
        );

        self.textures
            .insert(String::from("btn_exit"), (btn_exit_tex, btn_exit_pos));
        self.textures.insert(
            String::from("btn_exit_hover"),
            (btn_exit_hover_tex, btn_exit_pos),
        );

        Ok(())
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), UiError> {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        if self.has_current_game {
            if let Some((texture, position)) = self.textures.get(&self.current_btn_resume) {
                canvas
                    .copy(texture, None, *position)
                    .map_err(|_| UiError::SDL2Error)?;
            } else {
                return Err(UiError::MissingLoadedTexture);
            }
        }
        if let Some((texture, position)) = self.textures.get(&self.current_btn_new_game) {
            canvas
                .copy(texture, None, *position)
                .map_err(|_| UiError::SDL2Error)?;
        } else {
            return Err(UiError::MissingLoadedTexture);
        }
        if let Some((texture, position)) = self.textures.get(&self.current_btn_exit) {
            canvas
                .copy(texture, None, *position)
                .map_err(|_| UiError::SDL2Error)?;
        } else {
            return Err(UiError::MissingLoadedTexture);
        }

        canvas.present();

        Ok(())
    }

    fn update(&mut self, event: &Event) -> Result<ScreenOutcome, UiError> {
        match event {
            Event::MouseMotion { x, y, .. } => {
                // Draw buttons
                if self.has_current_game {
                    if self
                        .textures
                        .get("btn_resume")
                        .unwrap()
                        .1
                        .contains_point(Point::new(*x, *y))
                    {
                        self.current_btn_resume = String::from("btn_resume_hover");
                    } else {
                        self.current_btn_resume = String::from("btn_resume");
                    }
                }
                if self
                    .textures
                    .get("btn_new_game")
                    .unwrap()
                    .1
                    .contains_point(Point::new(*x, *y))
                {
                    self.current_btn_new_game = String::from("btn_new_game_hover");
                } else {
                    self.current_btn_new_game = String::from("btn_new_game");
                }
                if self
                    .textures
                    .get("btn_exit")
                    .unwrap()
                    .1
                    .contains_point(Point::new(*x, *y))
                {
                    self.current_btn_exit = String::from("btn_exit_hover");
                } else {
                    self.current_btn_exit = String::from("btn_exit");
                }
                return Ok(ScreenOutcome::Updated);
            }

            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                // Draw buttons
                if self.has_current_game
                    && self
                        .textures
                        .get("btn_resume")
                        .unwrap()
                        .1
                        .contains_point(Point::new(*x, *y))
                {
                    return Ok(ScreenOutcome::Resume);
                }
                if self
                    .textures
                    .get("btn_new_game")
                    .unwrap()
                    .1
                    .contains_point(Point::new(*x, *y))
                {
                    return Ok(ScreenOutcome::NewGame);
                } else if self
                    .textures
                    .get("btn_exit")
                    .unwrap()
                    .1
                    .contains_point(Point::new(*x, *y))
                {
                    return Ok(ScreenOutcome::Exit);
                }
            }
            _ => {}
        }

        Ok(ScreenOutcome::Unchanged)
    }
}

impl MainScreen {}
