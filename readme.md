# Jeu de Devinette de Nombre en Réseau

Ce projet est un jeu simple de "devinez le nombre" implémenté en Rust, où un serveur et plusieurs clients communiquent via un réseau. Les joueurs se connectent au serveur et tentent de deviner le nombre secret choisi par le serveur. 

## Règles du Jeu

1. Le serveur choisit un nombre secret entre 0 et 100.
2. Les joueurs se connectent au serveur et essaient de deviner le nombre.
3. Le serveur indique si le nombre deviné est correct ou non, et fournit des indices ("Trop bas" ou "Trop haut").
4. Chaque joueur dispose d'un maximum de 5 tentatives pour deviner le nombre.
5. Le jeu se déroule en tour par tour. Chaque joueur essaie de deviner le nombre à son tour.
6. Le premier joueur à deviner correctement le nombre gagne la partie.
7. Si aucun joueur ne devine le nombre après leurs 5 tentatives, le jeu se termine.

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation et Configuration

1. Clonez le dépôt du projet :
   ```bash
   git clone <URL_du_dépôt>
   cd <nom_du_dépôt>
   ```

2. Installez les dépendances nécessaires :
   ```bash
   cargo build
   ```

3. Créez un fichier `.env` à la racine du projet et ajoutez-y l'adresse IP du serveur :
   ```
   IP_SERVER=127.0.0.1:7878
   ```

## Commandes pour Lancer le Jeu

### Lancer le Serveur

```bash
cargo run --bin server
```

Cela lancera le serveur qui écoutera sur l'adresse spécifiée dans le fichier `.env`.

### Lancer le Client

```bash
cargo run --bin client
```

Cela lancera un client qui se connectera au serveur sur l'adresse spécifiée dans le fichier `.env` et permettra au joueur de participer au jeu.

## Fonctionnalités

- **Tour par tour** : Les joueurs devinent le nombre secret chacun à leur tour.
- **Messages d'attente** : Les joueurs sont informés lorsqu'ils doivent attendre leur tour.
- **Indications de devinette** : Le serveur informe si la tentative est trop basse, trop haute ou correcte.
- **Gestion des tentatives** : Chaque joueur dispose d'un maximum de 5 tentatives.
- **Fin de jeu** : Le jeu se termine lorsque le nombre est deviné ou lorsque les tentatives sont épuisées.

## Structure du Code

### `main.rs` (Serveur)

- **Fonctions** :
  - `handle_client` : Gère la communication avec un client.
  - `start_game` : Initialise une nouvelle partie.
  - `main` : Lance le serveur et accepte les connexions des clients.

### `client.rs` (Client)

- **Fonctions** :
  - `main` : Lance le client, se connecte au serveur et gère l'interaction avec le serveur.

### `game.rs`

- Contient les constantes de jeu et les structures principales (`GameState` et `Lobby`).
- `GameState` : Gère l'état global du jeu et les actions des joueurs (comme deviner un nombre).
- `Lobby` : Gère les joueurs en attente de commencer une partie.

### `lib.rs`

- Module principal du jeu.

### `tests/game_tests.rs`

- Contient des tests unitaires pour vérifier la logique du jeu (`GameState`).
