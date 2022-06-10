//! Player defined by four random AI modes
//!
//! Note: implementation here could be done in a more efficient way, but who
//! cares :-)
use chess_notation_parser::{Flag, FlagCheck, Piece, Turn};
use chess_turn_engine::ChessTurnEngine;
use rand::seq::SliceRandom;
use std::fmt;
use std::str::FromStr;

/// Representation of a random chess AI
pub enum Player {
    /// Player picks completely random turn
    Random,

    /// Aggressive player that will capture whenever possible.
    ///
    /// Turn priority order:
    /// 1. Checkmate
    /// 2. Capture or Check
    /// 3. Any other turn
    Aggressive,

    /// Pieceful player that never captures anything unless it is forced with
    /// check
    Pacifist,

    /// Pieceful player that has pawns that are only allowed to capture other
    /// pawns
    SemiPacifist,
}

impl Player {
    /// Find next turn
    pub fn decide_next_turn(&self, game: &ChessTurnEngine) -> String {
        const PAWN_CAN_ATTACK_PAWNS: bool = true;
        match self {
            Self::Random => find_random_turn(game),
            Self::Aggressive => find_aggressive_turn(game),
            Self::SemiPacifist => {
                find_pieceful_turn(game, PAWN_CAN_ATTACK_PAWNS)
            }
            Self::Pacifist => find_pieceful_turn(game, !PAWN_CAN_ATTACK_PAWNS),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Random => "Random",
                Self::Aggressive => "Aggressive",
                Self::Pacifist => "Pacifist",
                Self::SemiPacifist => "SemiPacifist",
            }
        )
    }
}

impl FromStr for Player {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Random" => Ok(Self::Random),
            "Aggressive" => Ok(Self::Aggressive),
            "Pacifist" => Ok(Self::Pacifist),
            "SemiPacifist" => Ok(Self::SemiPacifist),
            _ => Err("Invalid player input"),
        }
    }
}

/// Collect turns from list of `AvailableTurn`s
fn all_turns(game: &ChessTurnEngine) -> Vec<String> {
    game.available_turns()
        .iter()
        .map(|turn| String::from(turn.get_turn()))
        .collect::<Vec<String>>()
}

/// Pawn turns that contain capture of another pawn which are not checkmate
fn get_pawn_capture_other_pawns_turns(game: &ChessTurnEngine) -> Vec<String> {
    game.available_turns()
        .iter()
        .filter(|avail_turn| {
            match Turn::try_from(avail_turn.get_turn()).unwrap() {
                Turn::Move(turn) => {
                    turn.check_flag(Flag::CAPTURE)
                        && !turn.check_flag(Flag::CHECKMATE)
                        && turn.who == Piece::Pawn
                        && avail_turn.captured == Some(String::from("Pawn"))
                }
                _ => false,
            }
        })
        .map(|turn| String::from(turn.get_turn()))
        .collect::<Vec<String>>()
}

/// Peaceful include check and checkmate turns
fn get_all_pieceful_turns(game: &ChessTurnEngine) -> Vec<String> {
    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            !turn.is_capture()
        })
        .collect::<Vec<String>>()
}

/// All checkmate turns that do not capture
fn get_all_pieceful_checkmate_turns(game: &ChessTurnEngine) -> Vec<String> {
    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            turn.is_checkmate() && !turn.is_capture()
        })
        .collect::<Vec<String>>()
}

/// All checkmate turns
fn get_all_checkmate_turns(game: &ChessTurnEngine) -> Vec<String> {
    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            turn.is_checkmate()
        })
        .collect::<Vec<String>>()
}

/// Get all turns that result with a check
fn get_all_check_turns(game: &ChessTurnEngine) -> Vec<String> {
    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            turn.is_check()
        })
        .collect::<Vec<String>>()
}

/// Get all pieceful turns that do not contain check.
/// Prioritize checkmate turns if any
fn get_all_pieceful_with_check_turns(game: &ChessTurnEngine) -> Vec<String> {
    let checkmate = all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            !turn.is_capture() && !turn.is_check() && turn.is_checkmate()
        })
        .collect::<Vec<String>>();

    if checkmate.is_empty() {
        return checkmate;
    }

    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            !turn.is_capture() && !turn.is_check()
        })
        .collect::<Vec<String>>()
}

/// All capture turns that are not checkmate turns
fn get_capture_turns(game: &ChessTurnEngine) -> Vec<String> {
    all_turns(game)
        .into_iter()
        .filter(|turn| {
            let turn = Turn::try_from(turn.as_ref()).unwrap();
            turn.is_capture() && !turn.is_checkmate()
        })
        .collect::<Vec<String>>()
}

/// Randomly decide the most pieceful turn
fn find_pieceful_turn(
    game: &ChessTurnEngine,
    pawns_can_attack_pawns: bool,
) -> String {
    let turns = get_all_pieceful_checkmate_turns(game);
    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    let mut turns = get_all_pieceful_turns(game);

    if pawns_can_attack_pawns {
        let mut pawns_capture_turns = get_pawn_capture_other_pawns_turns(game);
        turns.append(&mut pawns_capture_turns);
    }

    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    let turns = get_all_pieceful_with_check_turns(game);
    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    let turns = get_capture_turns(game);
    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    all_turns(game)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
}

/// Randomly choose any turn that will harm other player
fn find_aggressive_turn(game: &ChessTurnEngine) -> String {
    let turns = get_all_checkmate_turns(game);
    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    // Combine check and capture turns together
    //
    // Because if we prioritize check turns, then often that
    // piece gets captured by the cornered enemy king
    let mut turns = get_all_check_turns(game);
    let mut capture_turns = get_capture_turns(game);
    turns.append(&mut capture_turns);
    if !turns.is_empty() {
        return turns.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    all_turns(game)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
}

/// Randomly choose any turn
fn find_random_turn(game: &ChessTurnEngine) -> String {
    all_turns(game)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
}
