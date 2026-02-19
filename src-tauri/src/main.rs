// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rakund_lib::setup::init;

fn main() {
    if let Err(e) = init::run() {
        eprintln!("Failed to start application: {:?}", e);
        std::process::exit(1);
    }
}
