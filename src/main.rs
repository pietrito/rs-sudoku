extern crate sdl2;
use sdl2::image::InitFlag;

mod cli;
mod errors;
mod game;
mod game_screen;
mod gui;
mod main_screen;
mod solver;
mod tests;
mod traits;
mod utils;

use std::env;

pub fn main() {
    // Get command line arguments and check there are 3
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        eprintln!(
            "This program should be launched as './sudocurs <CONFIGURATION_PATH> [CLI/GUI]'."
        );
        return;
    }
    if args.len() != 3 {
        eprintln!(
            "This program should be launched as '{} <CONFIGURATION_PATH> [CLI/GUI]'.",
            args[0]
        );
        return;
    }

    // Get the executable name
    let self_name = &args[0];
    println!("Launched [{}].", self_name);

    // The first argument should be the path to the configuration file
    let config_path = &args[1];
    println!("Loading configuration file [{}].", config_path);

    // The diplay mode, CLI for CLI and GUI for ggez graphics
    let mode: &str = match args[2].as_str() {
        "CLI" => "CLI",
        "GUI" => "GUI",
        _ => {
            eprintln!(
                "This program should be launched as '{} <CONFIGURATION_PATH> [CLI/GUI]'.\n
                Argument 2 should be one of 'CLI' or 'GUI', not '{}'.",
                self_name, args[2]
            );
            return;
        }
    };

    // Launch the game either in CLI or GUI mode
    match mode {
        "CLI" => {
            // Create the CLI using the configuration file
            let mut cli = match cli::Cli::new(config_path) {
                Ok(cli) => cli,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            // Play
            if let Err(e) = cli.run() {
                eprintln!("{}", e);
            }
        }
        "GUI" => {
            // Init SDL Context
            let sdl_context = sdl2::init().unwrap();
            // Init TTF Context
            let ttf_context = match sdl2::ttf::init() {
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
                Ok(ctx) => ctx,
            };
            // Init SDL Image Context
            if let Err(e) = sdl2::image::init(InitFlag::PNG) {
                eprintln!("{}", e);
                return;
            }

            // Create a GUI instance.
            let mut gui = match gui::Gui::new(&sdl_context, &ttf_context, config_path) {
                Ok(gui) => gui,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            // Try initiating the GUI
            if let Err(e) = gui.init() {
                eprintln!("{}", e);
                return;
            }

            // Launch the GUI
            if let Err(e) = gui.run() {
                eprintln!("{}", e);
            }
        }
        _ => panic!("WTF"),
    }
}
