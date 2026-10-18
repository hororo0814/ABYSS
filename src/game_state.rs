#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Title,
    Weapon,
    Enemy,
    Battle,
    Result,
    Prologue,
    Epilogue,
}

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Title,
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn transition_to(&mut self, next: GameState) {
        self.state = next;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::Game;
    use crate::game_state::GameState::{self, Weapon};

    #[test]
    fn first_game_status_test() {
        let game = Game::new();
        assert_eq!(game.state(), GameState::Title);
    }

    #[test]
    fn transe_game_status_test() {
        let mut game = Game::new();
        game.transition_to(Weapon);
        assert_eq!(game.state(), GameState::Weapon);
    }
}
