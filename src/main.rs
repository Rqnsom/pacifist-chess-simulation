//! Simple chess simulation with four `random` AI modes
//! - **Pacifist** - Always choose non-capture moves, if possible.
//!                  But do prioritize non-capture checkmate turns!
//! - **Aggressive** - Always prefer capture/check turns.
//!                    But only the checkmate turn has higher priorty.
//! - **SemiPacifist** - Pacifist mode where pawns are allowed to
//!                      capture other pawns.
//! - **Random** - Completely random turn decision from the pool of available
//!                turns.
//!
//! Not much else is there to say about it.
//! Use --help option to play around with this simulation.
//! Have fun and enjoy the random games.
use chess_turn_engine::{ChessTurnEngine, DisplayOption, Gamestate, Setup};
use clap::Parser;
use game_animator::GameAnimator;
use player::Player;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod game_animator;
mod player;

/// Simple chess simulation with four `random` AI modes
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// White player mode:
    /// Random/Pacifist/Aggressive/SemiPacifist
    #[clap(short = 'w', long, default_value_t = Player::Pacifist)]
    player_white: Player,

    /// White player mode:
    /// Random/Pacifist/Aggressive/SemiPacifist
    #[clap(short = 'b', long, default_value_t = Player::Pacifist)]
    player_black: Player,

    /// Animation frame duration in microseconds
    /// (animation lasts 20 frames)
    #[clap(short = 't', long, default_value_t = 50000)]
    animation_frame_duration: u64,
}

/// I guess this is where the fun starts
fn main() {
    let args = Args::parse();

    let black = args.player_black;
    let white = args.player_white;

    let animator = GameAnimator::new(args.animation_frame_duration);
    let mut game = ChessTurnEngine::new(Setup::Normal).unwrap();

    let running = setup_sig_handler();
    'outer: while running.load(Ordering::SeqCst) {
        for player in [&white, &black] {
            if game.gamestate() != Gamestate::Ongoing {
                break 'outer;
            }
            let turn = player.decide_next_turn(&game);
            if game_animator::DEBUG_ON {
                println!("\tPlaying: {}", turn);
            }
            animator.play_turn(&mut game, turn.as_str());
        }
    }
    game.display_on_screen(DisplayOption::TurnHistory);

    println!("\tWhite ({}) vs Black ({})", white, black);
    println!("\tGamestatus: {}", game.gamestate());

    if game_animator::DEBUG_ON {
        println!("\tTurn duration: {} us", animator.animation_duration());
        println!("\tFrame duration: {} us", animator.frame_duration());
    }
}

/// Print game status and turn history in case game is interrupted
/// (when DEBUG_ON is enabled)
fn setup_sig_handler() -> Arc<AtomicBool> {
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nEnding game forcefully");
    })
    .expect("Error setting Ctrl-C handler");

    running
}
