use number_game::game::{GameState, CORRECT_GUESS, TOO_LOW, TOO_HIGH, GAME_OVER_END, MAX_NUMBER_ATTEMPTS};

#[test]
fn test_correct_guess() {
    let mut game_state = GameState::new(2, 42);
    let response = game_state.make_guess(0, 42);
    assert_eq!(response, CORRECT_GUESS);
    assert_eq!(game_state.winner, Some(0));
    assert!(!game_state.active);
}

#[test]
fn test_too_low_guess() {
    let mut game_state = GameState::new(2, 42);
    let response = game_state.make_guess(0, 40);
    assert_eq!(response, TOO_LOW);
    assert_eq!(game_state.winner, None);
    assert!(game_state.active);
}

#[test]
fn test_too_high_guess() {
    let mut game_state = GameState::new(2, 42);
    let response = game_state.make_guess(0, 50);
    assert_eq!(response, TOO_HIGH);
    assert_eq!(game_state.winner, None);
    assert!(game_state.active);
}

#[test]
fn test_game_over_message() {
    let game_state = GameState::new(2, 42);
    assert_eq!(game_state.get_winner_message(0), GAME_OVER_END);
}

#[test]
fn test_max_attempts_reached() {
    let mut game_state = GameState::new(2, 42);
    game_state.attempts[0] = MAX_NUMBER_ATTEMPTS - 1;
    game_state.make_guess(0, 50);
    assert!(game_state.is_game_over());
    assert_eq!(game_state.get_winner_message(0), GAME_OVER_END);
}
