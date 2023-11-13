mod core;
mod assets;


fn init_logging() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}

fn main() {
    init_logging();

    log::info!("Starting the game");
    let mut game = core::game::Game::new().unwrap();
    game.start_game();
    log::info!("Game ended");
}