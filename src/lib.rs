use std::fmt;
use std::io;
use std::io::Write;
use rand::Rng;
use regex::Regex;
use rand::seq::SliceRandom;

pub fn print_help() {
    println!(
"\
Welcome to Toro y Vaca!\n
Rules:
 1) Each player has a secret 4-digits number
 2) The number doesn't starts with zero and doesn't repeat digits
 3) The first one to guess the opponent's number wins
 4) In his turn a player tell a guess, and the opponent gives feedback on the guess
 5) The feedback consists of the number of Toros (T) and Vacas(V)
 6) A Toro is a right digit in the right position
 7) A Vaca is a right digit in the wrong position
"
);
}

pub fn run() {
    println!("Toro y Vaca vs. AI mode");
    let mut human_player = HumanPlayer::new();
    let mut ai_player = AIPlayer::new();

    'gamloop: loop {
        println!();
        loop {
            match human_player.ask() {
                Some(guess) => {
                    let feedback = ai_player.give_feedback(guess);
                    println!("{}", feedback);
        
                    if feedback.info == Info::new(4, 0) {
                        println!("You won");
                        break 'gamloop;
                    }
                    break;
                },
                None => {
                    println!("Guess must be a 4 digit number, without repetitions.");
                    println!("Try again!");
                    continue;
                }
            }
        }

        println!();

        match ai_player.ask() {
            Some(guess) => {
                println!("What's in {}:", guess);
                let ans = human_player.give_feedback(guess);

                if ans.info == Info::new(4, 0) {
                    println!("Game Over!");
                    break;
                }

                ai_player.recieve_feedback(ans);
            },
            None => {
                println!("You lied to me!");
                break;
            }

        }
    }
}

pub fn run_guesser() {
    let human_player = HumanPlayer::new();
    let mut ai_player = AIPlayer::new();

    loop {
        println!();

        match ai_player.ask() {
            Some(guess) => {
                println!("What's in {}:", guess);
                let ans = human_player.give_feedback(guess);

                if ans.info == Info::new(4, 0) {
                    println!("Game Over!");
                    break;
                }

                ai_player.recieve_feedback(ans);
            },
            None => {
                println!("You lied to me!");
                break;
            }

        }
    }
}

pub struct Guess {
    val: [u16; 4]
}

impl Guess {
    pub fn generate() -> Guess {
        loop {
            let rand_int: u16 = rand::thread_rng().gen_range(1000..9999);
            if let Some(guess) = Guess::from(rand_int) {
                return guess;
            }
        }
    }

    pub fn compare(&self, guess: &Guess) -> Info {
        let mut toros = 0;
        let mut vacas = 0;

        for i in 0usize..=3usize {
            for j in 0..=3 {
                if self.val[i] == guess.val[j] {
                    if i == j {
                        toros += 1;
                    }
                    else {
                        vacas += 1;
                    }
                }
            }
        }
        Info::new(toros, vacas)
    }

    pub fn all() -> Vec<Guess> {
        let mut all_guesses: Vec<Guess> = Vec::new();

        for n in 1000u16..9999u16 {
            if let Some(guess) = Guess::from(n) {
                all_guesses.push(guess);
            }
        }

        all_guesses
    }

    pub fn from(n: u16) -> Option<Guess> {
        if 1000 < n && n < 9999 {
            let d = n % 10;
            let c = (n / 10) % 10;
            let b = (n / 100) % 10;
            let a = (n / 1000) % 10;

            if a != b && a != c && a != d && b != c && b != d && c != d {
                return Some(Guess{val: [a, b, c, d]});
            }
        }
        None
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}", self.val[0], self.val[1], self.val[2], self.val[3])
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Info {
    toros: u16,
    vacas: u16
}

impl Info {
    pub fn new(toros: u16, vacas: u16) -> Info {
        Info {toros, vacas}
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}T{}V", self.toros, self.vacas)
    }
}

pub struct Answer {
    guess: Guess,
    info: Info,
}

impl Answer {
    pub fn new(guess: Guess, info: Info) -> Answer {
        Answer{ guess, info }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.guess, self.info)
    }
}

pub trait Player {
    fn ask(&mut self) -> Option<Guess>;
    fn give_feedback(&self, guess: Guess) -> Answer;
    fn recieve_feedback(&mut self, ans: Answer);
}

struct HumanPlayer;

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        println!("Write down your number!\nPress 'Enter' to continue...");
        let mut discard_input = String::new();
        io::stdin()
            .read_line(&mut discard_input)
            .expect("Failed to read line");
        HumanPlayer {}
    }
}

impl Player for HumanPlayer {
    fn ask(&mut self) -> Option<Guess> {
        print!("Enter your guess: ");
        io::stdout().flush().expect("stdout().flush() shoud work");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u16>() {
            Ok(n) => {
                return Guess::from(n)
            }
            Err(_) => ()
        }
        
        None
    }

    fn give_feedback(&self, guess: Guess) -> Answer {
        let valid_format = Regex::new(r"^\dt\dv$").unwrap();

        loop {
            let mut info = String::new();
            io::stdin()
                .read_line(&mut info)
                .expect("Failed to read line");
            

            let info = info.trim().to_lowercase();

            if valid_format.is_match(&info) {
                //println!("{} {}", &info[0..1], &info[3..4]);
                let info = Info::new(
                    info[0..1].parse::<u16>().unwrap(),
                    info[2..3].parse::<u16>().unwrap()
                );

                return Answer::new(guess, info);
            }
            else {
                println!("Invalid format. Valid format is 1T2V or 1t2v");
                println!("Try again");
                continue;
            }
        }
    }

    fn recieve_feedback(&mut self, ans: Answer) {
        println!("{ans}");
    }
}

pub struct AIPlayer {
    secret_number: Guess,
    collected_info: Vec<Answer>,
    remaining_guesses: Vec<Guess>,
}

impl AIPlayer {
    pub fn new() -> AIPlayer {
        let mut guesses = Guess::all();
        guesses.shuffle(&mut rand::thread_rng());

        AIPlayer {
            secret_number: Guess::generate(),
            collected_info: Vec::new(),
            remaining_guesses: guesses
         }
    }

    pub fn remaining_guesses(&self) -> usize {
        self.remaining_guesses.len()
    }
}

impl Player for AIPlayer {
    fn ask(&mut self) -> Option<Guess> {
        self.remaining_guesses.pop()
    }

    fn give_feedback(&self, guess: Guess) -> Answer {
        let info = self.secret_number.compare(&guess);
        Answer::new(guess, info)
    }

    fn recieve_feedback(&mut self, ans: Answer) {
        self.remaining_guesses = self.remaining_guesses
            .drain(..)
            .filter(|g| g.compare(&ans.guess) == ans.info)
            .collect();
        self.collected_info.push(ans);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let info = Info::new(2, 1);
        let secret_number = Guess::from(1234).unwrap();
        let guess = Guess::from(2734).unwrap();
        assert_eq!(secret_number.compare(&guess), info);
    }
}