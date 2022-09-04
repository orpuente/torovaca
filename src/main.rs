use std::env;

fn main() {

    match mode(env::args()) {
        Mode::Guesser => torovaca::run_guesser(),
        Mode::Help => torovaca::print_help(),
        Mode::Normal => torovaca::run()
    }
}

enum Mode {
    Guesser,
    Help,
    Normal,
}

fn mode(mut args: impl Iterator<Item = String>) -> Mode {
    args.next(); // ignore program name

    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "--guess" => Mode::Guesser,
                "--help" => Mode::Help,
                _ => Mode::Normal
            }
        },
        None => Mode::Normal
    }
}