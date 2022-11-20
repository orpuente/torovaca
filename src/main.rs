use std::env;

fn main() {
    match mode(env::args()) {
        Mode::VisualGuesser => {
            println!("Visual Guess Mode");
            torovaca::run_guesser(true);
            torovaca::pause();
        }
        Mode::Guesser => {
            println!("Guess Mode");
            torovaca::run_guesser(false);
            torovaca::pause();
        },
        Mode::Help => torovaca::print_help(),
        Mode::Rules => torovaca::print_rules(),
        Mode::Normal => {
            println!("Normal Mode");
            torovaca::run();
            torovaca::pause();
        }
    }
}

enum Mode {
    VisualGuesser,
    Guesser,
    Help,
    Rules,
    Normal,
}

fn mode(mut args: impl Iterator<Item = String>) -> Mode {
    args.next(); // ignore program name

    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "--vguess" => Mode::VisualGuesser,
                "--guess" => Mode::Guesser,
                "--help" => Mode::Help,
                "--rules" => Mode::Rules,
                _ => Mode::Normal
            }
        },
        None => Mode::Normal
    }
}