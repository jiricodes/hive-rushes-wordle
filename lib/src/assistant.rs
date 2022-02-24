use crate::database::Database;
use crate::game::{LetterStatus, WordStatus};
use crate::suggestion::*;
use std::fmt::Debug;
use std::path::Path;

const MAX_SUGGESTIONS: usize = 25;
const COL_WIDTH: usize = 14;

pub struct Assistant {
    database: Database,
    suggestions: SuggestionCollection,
}

impl Assistant {
    /// Constructor
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let database = Database::load(filename);
        let suggestions = SuggestionCollection::from(database.get_available());
        Self {
            database,
            suggestions,
        }
    }

    /// Displays viable solutions
    pub fn display_suggestions(&self) {
        let len = self.database.get_len();
        let limit = len.min(MAX_SUGGESTIONS);
        println!("Showing {} out of {} suggestions", limit, len);
        println!(
            "{:<width$}{:<width$}{}",
            "Suggestion",
            "Unique chars",
            "Avg. frequency score",
            width = COL_WIDTH
        );
        let limit = MAX_SUGGESTIONS.min(self.suggestions.items.len());
        for i in 0..limit {
            let i = i as usize;
            self.suggestions.items[i].display();
        }
    }

    pub fn update(&mut self, input: &String, status_string: &String) {
        if self.database.available_contains(input) {
            self.database.discard(input);
        } else {
            println!("Word not found in available");
            return;
        }
        let wordstatus = WordStatus::from_strings(input, status_string);
        for (pos, ls) in wordstatus.iter().enumerate() {
            match ls {
                LetterStatus::Green(letter) => self.database.prune_green(*letter, pos),
                LetterStatus::Yellow(letter) => {
                    let cnt = wordstatus.char_count(*letter);
                    self.database.prune_n_yellow(*letter, pos, cnt.0 + cnt.1);
                }
                LetterStatus::Grey(letter) => {
                    let cnt = wordstatus.char_count(*letter);
                    self.database.prune_n_grey(*letter, cnt.0 + cnt.1);
                }
            }
        }
        let available = self.database.get_available();
        self.suggestions = SuggestionCollection::from(available);
        // Update the unique counter by updating with current guess' greens and yellows
        // Greens string is appended to the actual string then uniques are counted
        // this is a dirty way to create duplicates threfore reduce unique letters
        // TODO: add previous guesses
        let mut greens_and_yellows = wordstatus.get_green_chars();
        greens_and_yellows.extend(wordstatus.get_yellow_chars().chars());
        self.suggestions.update_with_info(&greens_and_yellows);
        self.suggestions.sort_suggestions_freq();
    }

    pub fn get_random(&mut self) -> Option<String> {
        self.suggestions.get_random_most_unique()
    }

    pub fn suggestions_empty(&self) -> bool {
        self.suggestions.items.is_empty()
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self {
            database: Database::new(),
            suggestions: SuggestionCollection::new(),
        }
    }
}
