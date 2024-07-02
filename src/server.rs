use std::sync::{Arc, Mutex};
use std::net::{TcpListener};
use crossbeam::thread::scope;

use crate::ui::handle_client;

pub fn run() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    let game = Arc::new(Mutex::new(crate::game::Game::new()));
    
    // Utilisation d'un pool de threads
    scope(|s| {
        for stream in listener.incoming() {
            let stream = stream.expect("failed to accept connection");
            let game = Arc::clone(&game);
            
            s.spawn(move |_| {
                handle_client(stream, game);
            });
        }
    }).expect("Thread pool failed");
}
