# Jeu de devinette de nombre en réseau

Ce projet consiste en un jeu simple de "devinez quel nombre je pense" implémenté en Rust avec une communication réseau entre un serveur et plusieurs clients. Les joueurs se connectent au serveur et tentent de deviner le nombre secret défini par le serveur.

## Règles du jeu

- Le serveur choisit un nombre secret entre 0 et 100.
- Les joueurs se connectent au serveur et envoient leurs tentatives pour deviner le nombre.
- Le serveur répond "OK" si le nombre est correct, sinon "ERR".
- Les joueurs ont un nombre limité de 5 tentatives pour deviner le nombre.
- Le jeu se déroule en tour par tour, chaque joueur essayant de deviner à son tour.

## Commandes pour lancer le jeu

### Lancer le serveur

```bash
cargo run --bin server
```

Cela lancera le serveur qui écoutera sur l'adresse `127.0.0.1:7878`.

### Lancer le client

```bash
cargo run --bin client
```

Cela lancera un client qui se connectera au serveur sur `127.0.0.1:7878` et permettra au joueur de jouer au jeu.

