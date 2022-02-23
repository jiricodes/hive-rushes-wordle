mod game_plugin;
use game_plugin::{Game, GameStatus};

use clap::{Arg, Command};

use std::io::{stdin, stdout, Write};

fn main() {
    let args = Command::new("add macro here")
        .arg(Arg::new("dict").index(1))
        .after_help("Words dictionary")
        .get_matches();
    let path = args
        .value_of("dict")
        .expect("dict file expected as argument");
    let mut game = Game::new(path);
    'gameloop: loop {
        let mut s = String::new();
        print!("Your Guess: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        match game.make_guess_simple(&s) {
            GameStatus::Ok(val) => println!("Status: {}", val),
            GameStatus::InvalidWord => println!("Guessed word not in dict"),
            GameStatus::GameOver => {
                println!("Game over, maximum attempts reached");
                break 'gameloop;
            }
            GameStatus::Victory => {
                println!("Congratulations, you guessed correct word");
                break 'gameloop;
            }
        }
    }
}
