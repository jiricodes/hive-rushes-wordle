use crate::letter_frequency::LetterFrequencyMap;
use indexmap::IndexSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::cmp::Ordering::Equal;

const COL_WIDTH: usize = 14;

pub struct SuggestionCollection {
	pub items: Vec<Suggestion>,
}

impl SuggestionCollection {
	/// Constructor
	pub fn new() -> Self {
		Self::default()
	}

	/// Sort based on n of unique chars
	pub fn sort_suggestions(&mut self) {
		self.items
			.sort_by(|a, b| b.unique_chars.cmp(&a.unique_chars));
	}

	/// Sort based on n of frequency score
	pub fn sort_suggestions_freq(&mut self) {
		self.items.sort_by(|a, b| {
			b.avg_frequency
				.partial_cmp(&a.avg_frequency)
				.unwrap_or(Equal)
		});
	}

	pub fn update_with_info(&mut self, info: &String) {
		for word in self.items.iter_mut() {
			word.update_unique_with_info(info);
			word.update_freq_with_info(info);
		}
	}

	pub fn max_unique(&mut self) -> i8 {
		self.sort_suggestions();
		self.items[0].get_unique_chars()
	}

	pub fn get_random_most_unique(&mut self) -> Option<String> {
		if self.items.is_empty() {
			return None;
		}
		let max_unique = self.max_unique();
		let mut max_arr: Vec<String> = Vec::new();
		for sugg in self.items.iter() {
			if sugg.get_unique_chars() >= max_unique {
				max_arr.push(sugg.get_word_clone())
			}
		}
		max_arr.choose(&mut rand::thread_rng()).cloned()
	}

	///gets the most frequent with most unique
	pub fn get_most_freq_uniq(&mut self) -> Option<String> {
		if self.items.is_empty() {
			return None;
		}
		let max_unique = self.max_unique();
		let mut max_arr: Vec<Suggestion> = Vec::new();
		for sugg in self.items.iter() {
			if sugg.get_unique_chars() >= max_unique {
				max_arr.push(sugg.clone())
			}
		}
		max_arr.sort_by(|a, b| {
			b.avg_frequency
				.partial_cmp(&a.avg_frequency)
				.unwrap_or(Equal)
		});
		Some(max_arr.get(0).unwrap().get_word_clone())
	}
}

impl From<&IndexSet<String>> for SuggestionCollection {
	fn from(available: &IndexSet<String>) -> Self {
		let mut suggestion_collection = SuggestionCollection::new();
		for word in available.iter() {
			let suggestion = Suggestion::new(word);
			suggestion_collection.items.push(suggestion);
		}
		suggestion_collection
	}
}

impl Default for SuggestionCollection {
	fn default() -> Self {
		Self { items: Vec::new() }
	}
}

#[derive(Clone)]
pub struct Suggestion {
	word: String,
	avg_frequency: f32,
	unique_chars: i8,
}

/// Lazy static LetterFrequencyMap
lazy_static! {
	static ref FREQ_MAP: LetterFrequencyMap = LetterFrequencyMap::new();
}

fn unique_char_count(word: &String) -> i8 {
	let chars: Vec<char> = word.chars().collect::<Vec<_>>();
	chars.into_iter().unique().count() as i8
}

fn count_avg_freq(word: &String) -> f32 {
	let mut sum: f32 = 0.0;
	for c in word.chars() {
		let val: &f32 = FREQ_MAP.get_frequency(c).unwrap_or(&0.0);
		sum += f32::from(*val);
	}
	sum / 5.0
}

impl Suggestion {
	/// Constructor
	pub fn new(word: &String) -> Self {
		Self {
			word: String::from(word),
			unique_chars: unique_char_count(word),
			avg_frequency: count_avg_freq(word),
		}
	}

	/// Display word
	pub fn display(&self) {
		println!(
			"{:<width$}{:<width$}{:<width$}",
			&self.word,
			&self.unique_chars,
			&self.avg_frequency,
			width = COL_WIDTH
		);
	}

	pub fn update_unique_with_info(&mut self, info: &String) {
		let s = self.word.clone() + info;
		self.unique_chars = unique_char_count(&s);
	}

	pub fn update_freq_with_info(&mut self, info: &String) {
		let mut positions: Vec<usize> = Vec::new();
		for c in info.chars() {
			positions.push(self.word.chars().position(|x| x == c).unwrap());
		}
		positions.sort();
		positions.reverse();
		// dbg!(&positions);
		let mut s = self.get_word_clone();
		for i in positions.iter() {
			s.remove(*i);
		}
		self.avg_frequency = count_avg_freq(&s);
	}

	pub fn get_unique_chars(&self) -> i8 {
		self.unique_chars
	}

	pub fn get_word_clone(&self) -> String {
		self.word.clone()
	}
}

impl Default for Suggestion {
	fn default() -> Self {
		Self {
			word: String::new(),
			avg_frequency: 0.0,
			unique_chars: 0,
		}
	}
}
