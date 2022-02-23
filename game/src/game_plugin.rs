use lib::database::Database;
use lib::game::{status_as_string, status_green, Wordle};
use std::fmt;
use std::fmt::Debug;
use std::path::Path;

/// Core Wordle Game struct
///
/// wordle provides the rules api and guess feedback
/// database can be used for word suggestions etc.
pub struct Game {
    wordle: Wordle,
    database: Database,
}

impl Game {
    /// Constructor that requires path to database
    ///
    /// TODO: change this perhaps to an object with `database` trait or similar
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let database = Database::load(filename);
        let word = database.get_random();
        println!("Wordle game with: {}", word);
        Self {
            wordle: Wordle::new(word),
            database,
        }
    }

    pub fn make_guess_simple(&mut self, word: &String) -> GameStatus<String> {
        if !self.database.contains(word) {
            return GameStatus::InvalidWord;
        }
        if self.wordle.game_over() {
            return GameStatus::GameOver;
        }
        let status = &self.wordle.guess_word(word);
        if status_green(&status) {
            return GameStatus::Victory;
        } else {
            GameStatus::Ok(status_as_string(&status))
        }
    }
}

pub enum GameStatus<T> {
    Ok(T),
    InvalidWord,
    GameOver,
    Victory,
}
