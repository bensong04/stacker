#![feature(let_chains)]
mod common;
mod repl;

use std::{cell::RefCell, env, fs, process, rc::Rc};
use toml::Value;

fn main() {
    let config_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <config.toml>", env::args().next().unwrap());
        process::exit(1);
    });

    let toml_str = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", config_path, e);
        process::exit(1);
    });

    let toml_val: Value = toml::from_str(&toml_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse {}: {}", config_path, e);
        process::exit(1);
    });
    let table = toml_val.as_table().unwrap_or_else(|| {
        eprintln!("Config file root is not a TOML table");
        process::exit(1);
    });

    let game = common::Game::from_config(table).unwrap_or_else(|e| {
        eprintln!("Improper TOML config: {}", e);
        process::exit(1);
    });

    let game_rc = Rc::new(RefCell::new(game));
    let mut repl = repl::new_game_repl(game_rc).unwrap_or_else(|e| {
        eprintln!("Failed to initialize REPL: {}", e);
        process::exit(1);
    });

    repl.run().expect("REPL crashed.");
}
