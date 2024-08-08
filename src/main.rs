mod game;
mod player;
mod util;
mod server;

fn main() {
    // Le point d'entrée du programme. Le serveur est démarré ici.
    server::run();
}
