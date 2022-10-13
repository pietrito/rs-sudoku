use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;

use std::rc::Rc;

use crate::errors::UiError;
use crate::game::Game;
use crate::traits::{Displayable, GUIConfig, ScreenOutcome};

const OFFSET_X: i32 = 40;
const OFFSET_Y: i32 = 40;
const BOX_SIZE: i32 = 35;

static COLOR_BCK: Color = Color::BLACK;
static COLOR_NOT_INIT: Color = Color::RGBA(75, 75, 75, 255);
// static COLOR_LINES: Color = Color::BLACK;
static COLOR_LINES: Color = Color::RGBA(255, 220, 0, 255);
static COLOR_HIGHLIGHT: Color = Color::RGBA(255, 110, 50, 255);
static COLOR_FONT: Color = Color::WHITE;
static _COLOR_GOOD_MSG: Color = Color::GREEN;
static _COLOR_BAD_MSG: Color = Color::RED;

#[derive(Default)]
pub struct GameScreen<'a> {
    pub game: Option<Game>,
    font: Option<Rc<Font<'a, 'a>>>,

    message: Option<String>,
}

impl<'a> GameScreen<'a> {
    /**
     * Returns wether or not the screen contains an undone game.
     */
    pub fn is_over(&self) -> bool {
        self.game.is_some() && self.game.as_ref().unwrap().is_done()
    }
}

impl<'a> Displayable for GameScreen<'a> {
    /**
     * Returns a new empty GameScreen instance. All its fields are set to `None`.
     */
    fn new() -> Self {
        GameScreen {
            ..Default::default()
        }
    }

    /**
     * If the configuration file contains the path of a game to load and resume.
     */
    fn init(
        &mut self,
        _canvas: &mut Canvas<sdl2::video::Window>,
        config: &GUIConfig,
    ) -> Result<(), UiError> {
        if !config.game_resume_path.is_empty() {
            self.game = Some(Game::from_file(&config.game_resume_path)?);
        }

        Ok(())
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), UiError> {
        // Reset screen with background color
        canvas.set_draw_color(COLOR_BCK);
        canvas.clear();

        // Drawing numbers
        for r in 0..self.game.as_ref().unwrap().side_size {
            for c in 0..self.game.as_ref().unwrap().side_size {
                // Getting the box's value only if it is not zero
                let number = match self.game.as_ref().unwrap().grid
                    [self.game.as_ref().unwrap().index(r, c)]
                .value
                {
                    0 => continue,
                    _ => &self.game.as_ref().unwrap().grid[self.game.as_ref().unwrap().index(r, c)],
                };

                // Choosing the background color of the box based on if its value is
                // highlighted or not
                if self.game.as_ref().unwrap().selected_value.is_some()
                    && *number == self.game.as_ref().unwrap().selected_value.unwrap()
                {
                    canvas.set_draw_color(COLOR_HIGHLIGHT);
                    canvas
                        .fill_rect(Rect::new(
                            OFFSET_X + (c as i32) * BOX_SIZE,
                            OFFSET_Y + (r as i32) * BOX_SIZE,
                            BOX_SIZE as u32,
                            BOX_SIZE as u32,
                        ))
                        .map_err(|_| UiError::SDL2Error)?;
                } else if number.value != 0 && !number.initial {
                    canvas.set_draw_color(COLOR_NOT_INIT);
                    canvas
                        .fill_rect(Rect::new(
                            OFFSET_X + (c as i32) * BOX_SIZE,
                            OFFSET_Y + (r as i32) * BOX_SIZE,
                            BOX_SIZE as u32,
                            BOX_SIZE as u32,
                        ))
                        .map_err(|_| UiError::SDL2Error)?;
                }

                let texture_creator = canvas.texture_creator();

                // Generating the number text
                let number_text = self
                    .font
                    .as_ref()
                    .unwrap()
                    .render(&number.value.to_string())
                    .solid(COLOR_FONT)
                    .map_err(|_| UiError::SDL2Error)?;

                let tex_number = texture_creator
                    .create_texture_from_surface(number_text)
                    .map_err(|_| UiError::SDL2Error)?;

                // Centering the number text in the box
                let offset_x = OFFSET_X + (BOX_SIZE - tex_number.query().width as i32) / 2 + 1;
                let offset_y = OFFSET_Y + (BOX_SIZE - tex_number.query().height as i32) / 2 + 1;

                canvas
                    .copy(
                        &tex_number,
                        None,
                        Rect::new(
                            offset_x + BOX_SIZE * c as i32,
                            offset_y + BOX_SIZE * r as i32,
                            tex_number.query().width,
                            tex_number.query().height,
                        ),
                    )
                    .map_err(|_| UiError::SDL2Error)?;
            }
        }

        // Drawing lines
        canvas.set_draw_color(COLOR_LINES);
        for n in 0..=self.game.as_ref().unwrap().side_size {
            // Line is thicker if modulo game size
            let thickness = match n % self.game.as_ref().unwrap().size {
                0 => 3,
                _ => 1,
            };

            // Horizontal line
            let line = Rect::new(
                OFFSET_X,
                OFFSET_Y + n as i32 * BOX_SIZE,
                self.game.as_ref().unwrap().side_size as u32 * BOX_SIZE as u32 + 3,
                thickness,
            );
            canvas.fill_rect(line).map_err(|_| UiError::SDL2Error)?;

            // Vertical line
            let line = Rect::new(
                OFFSET_X + n as i32 * BOX_SIZE,
                OFFSET_Y,
                thickness,
                self.game.as_ref().unwrap().side_size as u32 * BOX_SIZE as u32,
            );
            canvas.fill_rect(line).map_err(|_| UiError::SDL2Error)?;
        }

        /*
        if self.game.as_ref().unwrap().is_done() {
            let mut msg_text = Text::new("You won ! Congratulations !");
            // If possible, apply the loaded font to the error message
            if self.font_grid.is_some() {
                msg_text.set_font(*self.font_grid.as_ref().unwrap(), PxScale::from(30.0));
            }
            // Drawing the message
            graphics::draw(
                ctx,
                &msg_text,
                DrawParam::default()
                    .dest(mint::Point2 {
                        x: OFFSET_X,
                        y: OFFSET_Y * 2.0
                            + BOX_SIZE * (self.game.as_ref().unwrap().side_size as f32),
                    })
                    .color(*COLOR_GOOD_MSG),
            )?;
        }
        // If there is an error message to display
        else if self.message.is_some() {
            let mut msg_text = Text::new(self.message.as_ref().unwrap().as_str());
            // If possible, apply the loaded font to the error message
            if self.font_grid.is_some() {
                msg_text.set_font(*self.font_grid.as_ref().unwrap(), PxScale::from(30.0));
            }
            // Drawing the number
            graphics::draw(
                ctx,
                &msg_text,
                DrawParam::default()
                    .dest(mint::Point2 {
                        x: OFFSET_X,
                        y: OFFSET_Y * 2.0
                            + BOX_SIZE * (self.game.as_ref().unwrap().side_size as f32),
                    })
                    .color(*COLOR_BAD_MSG),
            )?;
        }
        */

        canvas.present();

        Ok(())
    }

    fn update(&mut self, event: &sdl2::event::Event) -> Result<ScreenOutcome, UiError> {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Num0 | Keycode::Num1),
                ..
            } => {}
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let x = *x;
                let y = *y;
                // If we're outside the grid, do nothing
                if x < OFFSET_X
                    || x >= OFFSET_X + (self.game.as_ref().unwrap().side_size as i32) * BOX_SIZE
                    || y < OFFSET_Y
                    || y >= OFFSET_Y + (self.game.as_ref().unwrap().side_size as i32) * BOX_SIZE
                {
                    if self.game.as_ref().unwrap().selected_index.is_some() {
                        self.game.as_mut().unwrap().selected_index = None;
                        self.game.as_mut().unwrap().selected_value = None;

                        return Ok(ScreenOutcome::Updated);
                    }
                    return Ok(ScreenOutcome::Unchanged);
                }

                // Calculate on which value the user clicked
                let row_index = ((y - OFFSET_Y) / BOX_SIZE) as usize;
                let col_index = ((x - OFFSET_X) / BOX_SIZE) as usize;
                let click_index = self.game.as_ref().unwrap().index(row_index, col_index);
                let click_value = self.game.as_ref().unwrap().grid[click_index].value;

                // If the game contains a number, highlight them, otherwise reset any highlighting
                if click_value == 0 && self.game.as_ref().unwrap().selected_value.is_some() {
                    let value = self.game.as_ref().unwrap().selected_value.unwrap();
                    match self
                        .game
                        .as_mut()
                        .unwrap()
                        .do_move(row_index, col_index, value)
                    {
                        Ok(_) => (),
                        Err(e) => {
                            self.message = Some(format!("{}", e));
                        }
                    }
                }

                self.game.as_mut().unwrap().selected_index = Some(click_index);
                self.game.as_mut().unwrap().selected_value = Some(click_value);
                return Ok(ScreenOutcome::Updated);
            }

            _ => {}
        }

        Ok(ScreenOutcome::Unchanged)
    }
}

impl<'a> GameScreen<'a> {
    pub fn set_game(&mut self, game: Game) {
        self.game = Some(game);
    }
    pub fn set_font(&mut self, new_font: Rc<Font<'a, 'a>>) {
        self.font = Some(new_font);
    }
    pub fn has_game(&self) -> bool {
        self.game.is_some()
    }
}
