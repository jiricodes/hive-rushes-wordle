use lib::database::Database;
use crate::suggestion::*;
use std::fmt::Debug;
use std::path::Path;

const MAX_SUGGESTIONS: i32 = 25;

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
        for i in 0..MAX_SUGGESTIONS {
            let i = i as usize;
            &self.suggestions.items[i].display();
        }
    }

    pub fn update(&mut self, input: &String) {
        //TODO: prune database based on available words and typed characters
        //TODO: update suggestions based on available words
        self.database.discard(input);
        let available = self.database.get_available();
        self.suggestions = SuggestionCollection::from(available);
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
