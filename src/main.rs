use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{env, mem, thread};
use std::time::Duration;
use rand::Rng;
use dotenv::dotenv;

const MAX_PLAYER_LOBBY: usize = 2;
const MIN_SECRET: i32 = 0;
const MAX_SECRET: i32 = 100;
const MAX_NUMBER_ATTEMPTS: usize = 5;

const WAITING_MESSAGE: &str = "Waiting for another player to make a guess...\n";
const GAME_OVER_WON: &str = "Game Over: You won the game!\n";
const GAME_OVER_END: &str = "Game Over: The game has ended.\n";
const GAME_OVER_MAX_ATTEMPTS: &str = "Game Over: You've reached the maximum number of attempts.\n";
const CORRECT_GUESS: &str = "Correct! You guessed the number!\n";
const TOO_LOW: &str = "Too low!\n";
const TOO_HIGH: &str = "Too high!\n";

struct GameState {
    secret_number: i32,
    current_turn: usize,
    attempts: Vec<usize>,
    active: bool,
    winner: Option<usize>,
}

struct Lobby {
    players: Vec<TcpStream>,
}

fn handle_client(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>, player_id: usize, sent_messages: Arc<Mutex<HashSet<usize>>>) {
    let welcome_message = format!(
        "Welcome to the game! You are Player {}. The number is between {} and {}.\n",
        player_id, MIN_SECRET, MAX_SECRET
    );
    stream.write(welcome_message.as_bytes()).expect("Failed to write to client");
    
    let mut buffer = [0; 512];
    let mut waiting_message_sent = false;

    loop {
        {
            let mut game_state = game_state.lock().unwrap();
            if !game_state.active {
                if let Some(winner) = game_state.winner {
                    if winner == player_id {
                        stream.write(GAME_OVER_WON.as_bytes()).expect("Failed to write to client");
                    } else {
                        let msg = format!("Game Over: Player {} won the game!\n", winner);
                        stream.write(msg.as_bytes()).expect("Failed to write to client");
                    }
                } else {
                    stream.write(GAME_OVER_END.as_bytes()).expect("Failed to write to client");
                }
                return;
            }

            if game_state.current_turn != player_id {
                if !waiting_message_sent {
                    stream.write(WAITING_MESSAGE.as_bytes()).expect("Failed to write to client");
                    waiting_message_sent = true;
                }
                drop(game_state);
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            waiting_message_sent = false;

            if game_state.attempts[player_id] >= MAX_NUMBER_ATTEMPTS {
                stream.write(GAME_OVER_MAX_ATTEMPTS.as_bytes()).expect("Failed to write to client");
                game_state.active = false;
                return;
            }

            let remaining_attempts = MAX_NUMBER_ATTEMPTS - game_state.attempts[player_id];
            let turn_message = format!(
                "It's your turn. Enter your guess: (Remaining attempts: {}) ",
                remaining_attempts
            );
            stream.write(turn_message.as_bytes()).expect("Failed to write to client");
        }

        let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");
        if bytes_read == 0 {
            return;
        }

        let guess: i32 = String::from_utf8_lossy(&buffer[..bytes_read]).trim().parse().unwrap_or(-1);
        let mut game_state = game_state.lock().unwrap();

        game_state.attempts[player_id] += 1;

        let response = if guess == game_state.secret_number {
            game_state.active = false;
            game_state.winner = Some(player_id);
            CORRECT_GUESS
        } else if guess < game_state.secret_number {
            TOO_LOW
        } else {
            TOO_HIGH
        };

        stream.write(response.as_bytes()).expect("Failed to write to client");

        if guess == game_state.secret_number {
            for attempt in &mut game_state.attempts {
                *attempt = MAX_NUMBER_ATTEMPTS;  // Set all attempts to max to effectively end the game for all players
            }
            game_state.active = false;
            break;
        }

        game_state.current_turn = (game_state.current_turn + 1) % game_state.attempts.len();
    }

    {
        let game_state = game_state.lock().unwrap();
        if let Some(winner) = game_state.winner {
            if winner == player_id {
                stream.write(GAME_OVER_WON.as_bytes()).expect("Failed to write to client");
            } else {
                let msg = format!("Game Over: Player {} won the game!\n", winner);
                stream.write(msg.as_bytes()).expect("Failed to write to client");
            }
        }
    }

    let mut sent_messages = sent_messages.lock().unwrap();
    sent_messages.remove(&player_id);
}

fn start_game(players: Vec<TcpStream>, sent_messages: Arc<Mutex<HashSet<usize>>>) {
    let secret_number = rand::thread_rng().gen_range(MIN_SECRET..=MAX_SECRET);
    let game_state = Arc::new(Mutex::new(GameState {
        secret_number,
        current_turn: 0,
        attempts: vec![0; players.len()],
        active: true,
        winner: None,
    }));

    println!("Starting a new game with secret number: {}", secret_number);

    for (player_id, stream) in players.into_iter().enumerate() {
        let game_state = Arc::clone(&game_state);
        let sent_messages = Arc::clone(&sent_messages);
        thread::spawn(move || {
            handle_client(stream, game_state, player_id, sent_messages);
        });
    }
}

fn main() {
    dotenv().ok();
    let ip_server = env::var("IP_SERVER").unwrap();
    let listener = TcpListener::bind(ip_server.clone()).expect("Failed to bind to address");
    println!("Server is running on {}", ip_server);

    let lobby = Arc::new(Mutex::new(Lobby { players: Vec::new() }));
    let sent_messages = Arc::new(Mutex::new(HashSet::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let lobby = Arc::clone(&lobby);
                let sent_messages = Arc::clone(&sent_messages);
                let mut lobby = lobby.lock().unwrap();
                let player_id = lobby.players.len();
                let message = format!(
                    "You are Player {}. You are in the lobby. There are currently {} players waiting.\n", 
                    player_id, player_id + 1
                );
                stream.write(message.as_bytes()).expect("Failed to write to client");
                lobby.players.push(stream.try_clone().expect("Failed to clone stream"));
                if lobby.players.len() == MAX_PLAYER_LOBBY {
                    let players = mem::replace(&mut lobby.players, Vec::new());
                    drop(lobby);
                    start_game(players, sent_messages);
                }
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
