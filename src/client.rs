use dotenv::dotenv;
use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    // Chargement des variables d'environnement depuis le fichier .env
    dotenv().ok();
    let ip_server = env::var("IP_SERVER").unwrap(); // Récupération de l'adresse IP du serveur depuis les variables d'environnement
    let mut stream = TcpStream::connect(&ip_server).expect("Could not connect to server"); // Connexion au serveur

    loop {
        let mut buffer = [0; 512];
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from server"); // Lecture des données envoyées par le serveur

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]); // Conversion des données en chaîne de caractères

            if response.contains("It's your turn") {
                let mut input = String::new();
                println!("{}", response); // Affichage du message du serveur
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input"); // Lecture de l'entrée utilisateur
                stream
                    .write_all(input.trim().as_bytes())
                    .expect("Failed to write to server"); // Envoi de la réponse au serveur
            } else {
                println!("{}", response); // Affichage du message du serveur
            }
        }
    }
}
