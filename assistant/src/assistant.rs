use crate::suggestion::*;
use lib::database::Database;
use lib::game::{LetterStatus, WordStatus};
use std::fmt::Debug;
use std::path::Path;

const MAX_SUGGESTIONS: usize = 25;
const COL_WIDTH: usize = 14;

pub struct Assistant {
    database: Database,
    suggestions: SuggestionCollection,
}

enum AssistantEnum {
    Grey(char),
    Green(char, usize),
    Yellow(char, usize),
}

impl Assistant {
    /// Constructor
    pub fn new<P>(filename: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let database = Database::load(filename);
        Self {
            database,
            ..Self::default()
        }
    }

    /// Displays viable solutions
    pub fn display_suggestions(&self) {
        let len = self.database.get_len();
        println!("Showing {} out of {} suggestions", MAX_SUGGESTIONS, len);
        println!(
            "{:<width$}{:<width$}",
            "Suggestion",
            "Unique chars",
            width = COL_WIDTH
        );
        let limit = MAX_SUGGESTIONS.min(self.suggestions.items.len());
        for i in 0..limit {
            let i = i as usize;
            self.suggestions.items[i].display();
        }
    }

    pub fn update(&mut self, input: &String, status_string: &String) {
        //TODO: prune database based on available words and typed characters
        //TODO: update suggestions based on available words
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
        self.suggestions.sort_suggestions();
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
