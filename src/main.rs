use voicepeak_cli::cli::{build_cli, handle_matches};
use voicepeak_cli::env_check::check_environment;

fn main() {
    if let Err(e) = check_environment() {
        eprintln!("Environment check failed: {}", e);
        std::process::exit(1);
    }

    let matches = build_cli().get_matches();

    if let Err(e) = handle_matches(matches) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
