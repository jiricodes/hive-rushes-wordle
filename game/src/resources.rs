use crate::components::*;
use crate::consts::*;
use bevy::prelude::*;
use lib::database::Database;
use lib::game::{status_green, LetterStatus, WordStatus, Wordle};
use std::fmt::Debug;
use std::path::Path;

/// Enum to express gamestatus
pub enum GameStatus<T> {
	Ok(T),
	InvalidWord,
	GameOver,
	Victory(T),
}

/// Core Wordle Game struct
///
/// wordle provides the rules api and guess feedback
/// database can be used for word suggestions etc.
pub struct Game {
	wordle: Wordle,
	database: Database,
	pub guesses: Vec<Option<String>>,
	pub colors: Vec<Vec<Color>>,
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
		let wordle = Wordle::new(word);
		let limit = wordle.get_max_attempts();
		Self {
			wordle,
			database,
			guesses: vec![None; limit],
			colors: vec![vec![TILE_DEFAULT_COLOR; 5]; limit],
		}
	}

	pub fn make_guess_simple(&mut self, word: &String) -> GameStatus<Vec<Color>> {
		if !self.database.contains(word) {
			return GameStatus::InvalidWord;
		}
		if self.wordle.game_over() {
			return GameStatus::GameOver;
		}
		let status = &self.wordle.guess_word(word);
		if status_green(&status) {
			return GameStatus::Victory(status_as_colors(&status));
		} else {
			GameStatus::Ok(status_as_colors(&status))
		}
	}

	pub fn reset(&mut self) {
		self.database.reset();
		let word = self.database.get_random();
		println!("Wordle game with: {}", word);
		self.wordle = Wordle::new(word);
		self.guesses = vec![None; self.wordle.get_max_attempts()];
		self.colors = vec![vec![TILE_DEFAULT_COLOR; 5]; self.wordle.get_max_attempts()];
	}
}

pub fn status_as_colors(status: &WordStatus) -> Vec<Color> {
	let mut colors: Vec<Color> = Vec::new();
	for ls in status.iter() {
		let color = match ls {
			LetterStatus::Grey => TILE_GREY_COLOR,
			LetterStatus::Yellow => TILE_YELLOW_COLOR,
			LetterStatus::Green => TILE_GREEN_COLOR,
		};
		colors.push(color);
	}
	colors
}

pub struct Cursor {
	pub position: TilePosition,
}

impl Default for Cursor {
	fn default() -> Self {
		Self {
			position: TilePosition { row: 0, col: 0 },
		}
	}
}

#[derive(Default)]
pub struct CurrentGuess {
	pub word: String,
}
