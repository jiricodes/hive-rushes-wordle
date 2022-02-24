use indexmap::IndexSet;
use itertools::Itertools;
use rand::seq::SliceRandom;

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

    pub fn update_with_info(&mut self, info: &String) {
        for word in self.items.iter_mut() {
            word.update_unique_with_info(info);
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

fn unique_char_count(word: &String) -> i8 {
    let chars: Vec<char> = word.chars().collect::<Vec<_>>();
    chars.into_iter().unique().count() as i8
}

pub struct Suggestion {
    word: String,
    probability: f32,
    unique_chars: i8,
}

impl Suggestion {
    /// Constructor
    pub fn new(word: &String) -> Self {
        Self {
            word: String::from(word),
            unique_chars: unique_char_count(word),
            ..Self::default()
        }
    }

    /// Display word
    pub fn display(&self) {
        println!(
            "{:<width$}{:<width$}",
            &self.word,
            &self.unique_chars,
            width = COL_WIDTH
        );
    }

    pub fn update_unique_with_info(&mut self, info: &String) {
        let s = self.word.clone() + info;
        self.unique_chars = unique_char_count(&s);
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
            probability: 0.0,
            unique_chars: 0,
        }
    }
}
