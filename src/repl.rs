use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

use crate::common::*;
use easy_repl::CommandStatus;
use easy_repl::{Repl, command};

#[derive(Debug)]
struct ReplError(String);

impl Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.0)
    }
}

impl Error for ReplError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

pub fn new_game_repl<'a>(game: Rc<RefCell<Game>>) -> Result<Repl<'a>, &'static str> {
    // pain
    let game1 = game.clone();
    let game2 = game.clone();
    let game3 = game.clone();
    let game4 = game.clone();
    let game5 = game.clone();
    let game6 = game.clone();
    let game7 = game.clone();
    let game8 = game.clone();
    let game9 = game.clone();

    let repl = Repl::builder()
        .prompt("♦ ")
        .description("Poker ledger tracker utility for home games.")
        .add(
            "sit",
            command! {
                "Add a new player with some buyin",
                (name: String, buyin: i64) => |name, buyin| {
                    game1.borrow_mut().add_player(name, buyin, false).map_err(|s| ReplError(s))?;
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "host",
            command! {
                "Add a new player with host privileges and some buyin",
                (name: String, buyin: i64) => |name, buyin| {
                    game2.borrow_mut().add_player(name, buyin, true).map_err(|s| ReplError(s))?;
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "addon",
            command! {
                "Record that a player added on for some amount",
                (name: String, addon: i64) => |name, addon| {
                    game3.borrow_mut().addon_player(&name, addon).map_err(|s| ReplError(s))?;
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "check",
            command! {
                "Display player info without removing the player from the game",
                (name: String) => |name| {
                    let mut binding = game4.borrow_mut();
                    let player = binding.get_player(&name).map_err(|s| ReplError(s))?;
                    println!("{}", player);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "cashout",
            command! {
                "Remove the player from the game and display final stats",
                (name: String, stack: i64) => |name, stack| {
                    let mut binding = game5.borrow_mut();
                    let player = binding.get_player(&name).map_err(|s| ReplError(s))?;
                    println!("{}", player);
                    let info = binding.cashout_player(&name, stack).map_err(|s| ReplError(s))?;
                    println!("\nSUMMARY:\nBUYIN: ${} | CASHOUT: ${} | LEDGER: ${} | PAID RAKE: ${}", info.0, info.1, info.2, info.3);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "rem",
            command! {
                "Remove the player from the game without displaying final stats",
                (name: String, stack: i64) => |name, stack| {
                    let info = game6.borrow_mut().cashout_player(&name, stack).map_err(|s| ReplError(s))?;
                    println!("\nSUMMARY:\nBUYIN: ${} | CASHOUT: ${} | LEDGER: ${} | PAID RAKE: ${}", info.0, info.1, info.2, info.3);
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "total",
            command! {
                "Display the amount of money on the table",
                () => || {
                    println!("${}", game7.borrow().total_money());
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "summary",
            command! {
                "Display a comprehensive summary of the game right now",
                () => || {
                    println!("{}", game8.borrow());
                    Ok(CommandStatus::Done)
                }
            },
        )
        .add(
            "end",
            command! {
                "Summarizes the game. Cashes everyone out. Quits. (Manual cashouts are preferred.)",
                () => || {
                    println!("{}", game9.borrow());
                    Ok(CommandStatus::Quit)
                }
            },
        )
        .build().expect("Failed to build REPL");

    Ok(repl)
}
