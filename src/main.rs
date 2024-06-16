use rustifly_engine::{manager::log_manager, MANAGERS};

fn main() {
    let log_manager = MANAGERS.lock().unwrap().take_log();
    log_manager.write_log(log_manager::LogKind::INFO, "LETSGOOOOOOOOOOOOO".to_string());

    let mut game_manager = MANAGERS.lock().unwrap().take_game();

    game_manager.run();
}
