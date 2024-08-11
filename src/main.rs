use dotenv::dotenv;
use number_game::game::{
    self, GameState, Lobby, GAME_OVER_END, GAME_OVER_MAX_ATTEMPTS, GAME_OVER_WON, MAX_PLAYER_LOBBY,
    WAITING_MESSAGE,
};
use rand::Rng;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, mem, thread};

// Fonction pour gérer les interactions avec chaque client.
fn handle_client(
    mut stream: TcpStream,
    game_state: Arc<Mutex<GameState>>,
    player_id: usize,
    sent_messages: Arc<Mutex<HashSet<usize>>>,
) {
    // Message de bienvenue pour le joueur avec les informations nécessaires.
    let welcome_message = format!(
        "Welcome to the game! You are Player {}. The number is between {} and {}.\n",
        player_id,
        game::MIN_SECRET,
        game::MAX_SECRET
    );
    stream
        .write(welcome_message.as_bytes())
        .expect("Failed to write to client");

    let mut buffer = [0; 512];
    let mut waiting_message_sent = false;

    loop {
        {
            // Verrouiller l'état du jeu pour accéder et modifier les données partagées.
            let mut game_state = game_state.lock().unwrap();

            // Vérifier si le jeu est terminé et envoyer le message approprié.
            if !game_state.active {
                if let Some(winner) = game_state.winner {
                    if winner == player_id {
                        stream
                            .write(GAME_OVER_WON.as_bytes())
                            .expect("Failed to write to client");
                    } else {
                        let msg = format!("Game Over: Player {} won the game!\n", winner);
                        stream
                            .write(msg.as_bytes())
                            .expect("Failed to write to client");
                    }
                } else {
                    stream
                        .write(GAME_OVER_END.as_bytes())
                        .expect("Failed to write to client");
                }
                return;
            }

            // Si ce n'est pas le tour du joueur, envoyer un message d'attente.
            if game_state.current_turn != player_id {
                if !waiting_message_sent {
                    stream
                        .write(WAITING_MESSAGE.as_bytes())
                        .expect("Failed to write to client");
                    waiting_message_sent = true;
                }
                drop(game_state);
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            waiting_message_sent = false;

            // Vérifier si le joueur a atteint le nombre maximum de tentatives.
            if game_state.attempts[player_id] >= game::MAX_NUMBER_ATTEMPTS {
                stream
                    .write(GAME_OVER_MAX_ATTEMPTS.as_bytes())
                    .expect("Failed to write to client");
                game_state.active = false;
                return;
            }

            // Envoyer un message pour inviter le joueur à entrer sa devinette.
            let remaining_attempts = game::MAX_NUMBER_ATTEMPTS - game_state.attempts[player_id];
            let turn_message = format!(
                "It's your turn. Enter your guess: (Remaining attempts: {}) ",
                remaining_attempts
            );
            stream
                .write(turn_message.as_bytes())
                .expect("Failed to write to client");
        }

        // Lire la devinette envoyée par le client.
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from client");
        if bytes_read == 0 {
            return; // Si aucune donnée n'est lue, le client s'est déconnecté.
        }

        // Convertir la devinette en un nombre entier.
        let guess: i32 = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .parse()
            .unwrap_or(-1);
        let mut game_state = game_state.lock().unwrap();

        // Faire une devinette et recevoir la réponse appropriée.
        let response = game_state.make_guess(player_id, guess);
        stream
            .write(response.as_bytes())
            .expect("Failed to write to client");

        // Vérifier si le jeu est terminé après cette devinette.
        if game_state.is_game_over() {
            game_state.active = false;
            for attempt in &mut game_state.attempts {
                *attempt = game::MAX_NUMBER_ATTEMPTS; // Met fin au jeu pour tous les joueurs.
            }
            break;
        }

        // Passer le tour au joueur suivant.
        game_state.current_turn = (game_state.current_turn + 1) % game_state.attempts.len();
    }

    // Envoyer un message final au joueur indiquant le résultat du jeu.
    let game_state = game_state.lock().unwrap();
    let msg = game_state.get_winner_message(player_id);
    stream
        .write(msg.as_bytes())
        .expect("Failed to write to client");

    let mut sent_messages = sent_messages.lock().unwrap();
    sent_messages.remove(&player_id);
}

// Fonction pour démarrer une nouvelle partie avec les joueurs connectés.
fn start_game(players: Vec<TcpStream>, sent_messages: Arc<Mutex<HashSet<usize>>>) {
    // Générer un nombre secret aléatoire pour cette partie.
    let secret_number = rand::thread_rng().gen_range(game::MIN_SECRET..=game::MAX_SECRET);
    let game_state = Arc::new(Mutex::new(GameState::new(players.len(), secret_number)));

    println!("Starting a new game with secret number: {}", secret_number);

    // Créer un thread pour chaque joueur pour gérer les interactions du client.
    for (player_id, stream) in players.into_iter().enumerate() {
        let game_state = Arc::clone(&game_state);
        let sent_messages = Arc::clone(&sent_messages);
        thread::spawn(move || {
            handle_client(stream, game_state, player_id, sent_messages);
        });
    }
}

fn main() {
    dotenv().ok(); // Charger les variables d'environnement depuis le fichier `.env`.
    let ip_server = env::var("IP_SERVER").unwrap(); // Lire l'adresse IP du serveur.
    let listener = TcpListener::bind(ip_server.clone()).expect("Failed to bind to address"); // Lier le serveur à l'adresse IP.
    println!("Server is running on {}", ip_server);

    let lobby = Arc::new(Mutex::new(Lobby {
        players: Vec::new(),
    })); // Créer un lobby pour gérer les joueurs en attente.
    let sent_messages = Arc::new(Mutex::new(HashSet::new())); // Utilisé pour gérer les messages envoyés aux joueurs.

    // Accepter les connexions entrantes des clients.
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let lobby = Arc::clone(&lobby);
                let sent_messages = Arc::clone(&sent_messages);
                let mut lobby = lobby.lock().unwrap();
                let player_id = lobby.players.len();

                // Envoyer un message au client indiquant sa position dans le lobby.
                let message = format!(
                    "You are Player {}. You are in the lobby. There are currently {} players waiting.\n", 
                    player_id, player_id + 1
                );
                stream
                    .write(message.as_bytes())
                    .expect("Failed to write to client");

                // Ajouter le joueur au lobby.
                lobby
                    .players
                    .push(stream.try_clone().expect("Failed to clone stream"));

                // Si le nombre maximum de joueurs est atteint, démarrer une nouvelle partie.
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
