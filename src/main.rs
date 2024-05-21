use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

struct GameState {
    secret_number: i32,
    current_turn: usize,
    attempts: Vec<usize>,
    active: bool,
}

struct Lobby {
    players: Vec<TcpStream>,
}

fn handle_client(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>, player_id: usize, sent_messages: Arc<Mutex<HashSet<usize>>>) {
    stream.write(b"Welcome to the game! The number is between 0 and 100.\n").expect("Failed to write to client");

    let mut buffer = [0; 512];
    loop {
        {
            let mut game_state = game_state.lock().unwrap();
            if !game_state.active {
                stream.write(b"Game Over: The game has ended.\n").expect("Failed to write to client");
                return;
            }

            if game_state.current_turn != player_id {
                stream.write(b"Waiting for another player to make a guess...\n").expect("Failed to write to client");
                drop(game_state);
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }

            if game_state.attempts[player_id] >= 5 {
                stream.write(b"Game Over: You've reached the maximum number of attempts.\n").expect("Failed to write to client");
                game_state.active = false;
                return;
            }

            stream.write(b"It's your turn. Enter your guess: ").expect("Failed to write to client");
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
            "OK\n"
        } else {
            "ERR\n"
        };

        stream.write(response.as_bytes()).expect("Failed to write to client");

        if guess == game_state.secret_number {
            for attempt in &mut game_state.attempts {
                *attempt = 5;  // Set all attempts to max to effectively end the game for all players
            }
            game_state.active = false;
            break;
        }

        game_state.current_turn = (game_state.current_turn + 1) % game_state.attempts.len();
    }

    let mut sent_messages = sent_messages.lock().unwrap();
    sent_messages.remove(&player_id);
}

fn start_game(players: Vec<TcpStream>, sent_messages: Arc<Mutex<HashSet<usize>>>) {
    let secret_number = rand::thread_rng().gen_range(0..=100);
    let game_state = Arc::new(Mutex::new(GameState {
        secret_number,
        current_turn: 0,
        attempts: vec![0; players.len()],
        active: true,
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
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server is running on 127.0.0.1:7878");

    let lobby = Arc::new(Mutex::new(Lobby { players: Vec::new() }));
    let sent_messages = Arc::new(Mutex::new(HashSet::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let lobby = Arc::clone(&lobby);
                let sent_messages = Arc::clone(&sent_messages);
                thread::spawn(move || {
                    let mut lobby = lobby.lock().unwrap();
                    let player_count = lobby.players.len();
                    let message = format!("You are in the lobby. There are currently {} players waiting.\n", player_count + 1);
                    stream.write(message.as_bytes()).expect("Failed to write to client");
                    lobby.players.push(stream.try_clone().expect("Failed to clone stream"));
                    if lobby.players.len() == 5 {
                        let players = std::mem::replace(&mut lobby.players, Vec::new());
                        drop(lobby);
                        start_game(players, sent_messages);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
