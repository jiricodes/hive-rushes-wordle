//! Word database module
//!

use indexmap::IndexSet;
use rand::{thread_rng, Rng};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Main database struct
///
/// Using IndexSet as opposed to HashSet. The [IndexSet](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html)
/// allows us to efficiently random sample the words.
/// Using HashSet like structure helps discarding possible
/// duplicates within the given dict.
///
/// WIP
/// perhaps we dont need discarded, however lets keep if for now
#[derive(Debug)]
pub struct Database {
	available: IndexSet<String>,
	discarded: IndexSet<String>,
}

impl Database {
	/// Simple constructor, returns default aka empty
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates the database from a file
	///
	/// Panics on file opening error or line reading error
	///
	/// # Example
	/// ```
	/// use lib::database::Database;
	///
	/// let d = Database::load("../data/test_dict.txt");
	/// dbg!(d);
	/// ```
	pub fn load<P>(filename: P) -> Self
	where
		P: AsRef<Path> + Debug,
	{
		let file = File::open(filename).expect("File error");
		let reader = BufReader::new(file);
		let mut all_words: IndexSet<String> = IndexSet::new();
		for line in reader.lines() {
			all_words.insert(line.expect("line error").to_lowercase());
		}
		Self {
			available: all_words,
			..Default::default()
		}
	}

	/// Returns random word from the available set
	/// Does not consume.
	pub fn get_random(&self) -> String {
		let mut rng = thread_rng();
		let i = rng.gen_range(0..self.available.len()) as usize;
		self.available.get_index(i).unwrap().clone()
	}

	/// Moves given word from available to discarded
	/// Panics if word not in the available set
	fn discard(&mut self, word: &String) {
		let w = self
			.available
			.take(word)
			.expect("Word not in available set");
		self.discarded.insert(w);
	}

	/// Removes all words from the available set that DON'T HAVE given `letter` at specified `position`
	fn prune_letter_at_position(&mut self, letter: char, position: usize) {
		let mut to_prune: IndexSet<String> = IndexSet::new();
		for word in self.available.iter() {
			let c = word.chars().nth(position).expect("postion out of index");
			if c != letter {
				to_prune.insert(word.clone());
			}
		}
		for word in to_prune.iter() {
			self.discard(word);
		}
	}

	/// Removes all words from the available set that HAVE given `letter` at specified `position`
	/// This can be used when guessed correct letter, but at wrong position
	fn prune_letter_at_incorrect_position(&mut self, letter: char, position: usize) {
		let mut to_prune: IndexSet<String> = IndexSet::new();
		for word in self.available.iter() {
			let c = word.chars().nth(position).expect("postion out of index");
			if c == letter {
				to_prune.insert(word.clone());
			}
		}
		for word in to_prune.iter() {
			self.discard(word);
		}
	}

	/// Removes all words that don't contain given `letter`.
	fn prune_letter_any_position(&mut self, letter: char) {
		let mut to_prune: IndexSet<String> = IndexSet::new();
		for word in self.available.iter() {
			if !word.contains(letter) {
				to_prune.insert(word.clone());
			}
		}
		for word in to_prune.iter() {
			self.discard(word);
		}
	}

	/// Removes all words that contain specific letter
	fn prune_letter(&mut self, letter: char) {
		let mut to_prune: IndexSet<String> = IndexSet::new();
		for word in self.available.iter() {
			if word.contains(letter) {
				to_prune.insert(word.clone());
			}
		}
		for word in to_prune.iter() {
			self.discard(word);
		}
	}

	/// Available words getter (pointer)
	pub fn get_available(&self) -> &IndexSet<String> {
		&self.available
	}

	/// GREEN status - aka correct letter at correct position
	///
	/// This results in moving all words that do not have given
	/// `letter` at the specific `position` from available set to
	/// discarded set.
	pub fn prune_green(&mut self, letter: char, position: usize) {
		self.prune_letter_at_position(letter, position);
	}

	/// YELLOW status - aka  correct letter at incorrect position
	///
	/// This results in moving all words that HAVE given
	/// `letter` at the specific `position` from available set to
	/// discarded set. Also all words that do not contain `letter` at all.
	pub fn prune_yellow(&mut self, letter: char, position: usize) {
		self.prune_letter_at_incorrect_position(letter, position);
		self.prune_letter_any_position(letter);
	}

	/// GREY status - aka incorrect letter
	///
	/// This results in moving all words that contain given
	/// `letter` from available set to
	/// discarded set.
	pub fn prune_grey(&mut self, letter: char) {
		self.prune_letter(letter);
	}

	/// Checks if given `word` is in the database
	/// either available or discarded
	pub fn contains(&self, word: &String) -> bool {
		self.available_contains(word) || self.discarded_contains(word)
	}

	/// Checks if the `available` set contains given `word`
	fn available_contains(&self, word: &String) -> bool {
		self.available.contains(word)
	}

	/// Checks if the `discarded` set contains given `word`
	fn discarded_contains(&self, word: &String) -> bool {
		self.discarded.contains(word)
	}

	/// Returns discarded back to available set.
	pub fn reset(&mut self) {
		self.available.extend(self.discarded.clone());
		self.discarded.clear();
	}
}

impl Default for Database {
	fn default() -> Self {
		Self {
			available: IndexSet::new(),
			discarded: IndexSet::new(),
		}
	}
}
