use entering_sleep_mode_command::run_interactive;

fn main() {
    if let Err(err) = run_interactive() {
        eprintln!("Engine error: {err}");
        std::process::exit(1);
    }
}
