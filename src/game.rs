use std::net::TcpStream;

pub const MAX_PLAYER_LOBBY: usize = 2;
pub const MIN_SECRET: i32 = 0;
pub const MAX_SECRET: i32 = 100;
pub const MAX_NUMBER_ATTEMPTS: usize = 5;

pub const WAITING_MESSAGE: &str = "Waiting for another player to make a guess...\n";
pub const GAME_OVER_WON: &str = "Game Over: You won the game!\n";
pub const GAME_OVER_END: &str = "Game Over: The game has ended.\n";
pub const GAME_OVER_MAX_ATTEMPTS: &str = "Game Over: You've reached the maximum number of attempts.\n";
pub const CORRECT_GUESS: &str = "Correct! You guessed the number!\n";
pub const TOO_LOW: &str = "Too low!\n";
pub const TOO_HIGH: &str = "Too high!\n";

pub struct GameState {
    pub secret_number: i32,
    pub current_turn: usize,
    pub attempts: Vec<usize>,
    pub active: bool,
    pub winner: Option<usize>,
}

impl GameState {
    pub fn new(num_players: usize, secret_number: i32) -> Self {
        GameState {
            secret_number,
            current_turn: 0,
            attempts: vec![0; num_players],
            active: true,
            winner: None,
        }
    }

    pub fn make_guess(&mut self, player_id: usize, guess: i32) -> &'static str {
        self.attempts[player_id] += 1;

        if self.attempts[player_id] >= MAX_NUMBER_ATTEMPTS {
            if self.attempts.iter().all(|&attempt| attempt >= MAX_NUMBER_ATTEMPTS) {
                self.active = false;
            }
            return GAME_OVER_MAX_ATTEMPTS;
        }

        if guess == self.secret_number {
            self.active = false;
            self.winner = Some(player_id);
            CORRECT_GUESS
        } else if guess < self.secret_number {
            TOO_LOW
        } else {
            TOO_HIGH
        }
    }

    pub fn is_game_over(&self) -> bool {
        !self.active || self.attempts.iter().all(|&attempt| attempt >= MAX_NUMBER_ATTEMPTS)
    }

    pub fn get_winner_message(&self, player_id: usize) -> String {
        if let Some(winner) = self.winner {
            if winner == player_id {
                GAME_OVER_WON.to_string()
            } else {
                format!("Game Over: Player {} won the game!\n", winner)
            }
        } else {
            GAME_OVER_END.to_string()
        }
    }
}

pub struct Lobby {
    pub players: Vec<TcpStream>,
}
