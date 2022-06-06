use chess_turn_engine::{ChessTurnEngine, DisplayOption, ViewMode};
use std::{thread, time};

/// We should definitely move this someplace else
pub const DEBUG_ON: bool = false;

/// `GameAnimator` plays the turn while performing simple animations
pub struct GameAnimator {
    /// Animation lasting time
    animation_duration_time: u64,

    /// Animation frame duration
    frame_duration: u64,

    /// Number of animation play/undo cycles
    animation_cycles: u64,

    /// Sleep time when `undo turn` is done in animation
    t_undo: u64,

    /// Sleep time when `play turn` is done in animation
    t_play: u64,

    /// Sleep time after animation is over
    t_turn_done: u64,
}

impl GameAnimator {
    /// Create new `GameAnimator`
    pub fn new(frame_duration: u64) -> Self {
        assert_ne!(frame_duration, 0, "Turn duration mustn't be zero!");

        // Do not mess around with my numbers :-)
        let t_undo = frame_duration * 2;
        let t_play = frame_duration * 3;
        let t_turn_done = frame_duration * 5;

        let animation_cycles = 3;
        let total_t = animation_cycles * (t_undo + t_play) + t_turn_done;

        Self {
            animation_duration_time: total_t,
            frame_duration,
            animation_cycles,
            t_undo,
            t_play,
            t_turn_done,
        }
    }

    /// Get animation duration
    pub fn animation_duration(&self) -> u64 {
        self.animation_duration_time
    }

    /// Get animation frame duration
    pub fn frame_duration(&self) -> u64 {
        self.frame_duration
    }

    /// Play the turn and animate it simply
    pub fn play_turn(&self, game: &mut ChessTurnEngine, turn: &str) {
        let fancy_display = DisplayOption::BoardView(ViewMode::FancyTui);

        // Record display state before playing the turn
        let before_turn = game.display(fancy_display);

        if let Err(e) = game.play_turn(&turn) {
            panic!("{}", format!("Unexpected error: {}", e).as_str())
        }

        let after_turn = game.display(fancy_display);
        let capture_history = game.display(DisplayOption::CaptureHistory);

        for _ in 0..self.animation_cycles {
            println!("{}{}", before_turn, capture_history);
            thread::sleep(time::Duration::from_micros(self.t_undo));

            println!("{}{}", after_turn, capture_history);
            thread::sleep(time::Duration::from_micros(self.t_play));
        }

        if DEBUG_ON {
            let turn_history = game.display(DisplayOption::TurnHistory);
            println!("{}", turn_history);
        }

        thread::sleep(time::Duration::from_micros(self.t_turn_done));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess_turn_engine::{ChessTurnEngine, Setup};

    #[test]
    fn basic_functionality_works() {
        let duration = 100;
        let animator = GameAnimator::new(duration);

        assert_eq!(animator.frame_duration(), duration);

        // Manually calculated :-)
        assert_eq!(animator.animation_duration(), duration * 20);

        let mut game = ChessTurnEngine::new(Setup::Normal).unwrap();
        // Let's play two turns
        animator.play_turn(&mut game, "d4");
        animator.play_turn(&mut game, "d5");
    }

    #[test]
    #[should_panic]
    fn invalid_duration_causes_suicide() {
        let duration = 0;
        let _animator = GameAnimator::new(duration);
    }
}
