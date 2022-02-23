use lib::database::Database;
use crate::suggestion::*;

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
    pub fn new() -> Self {
        Self::default()
    }

    /// 
    pub fn display_suggestions(&self) {
        for i in 0..MAX_SUGGESTIONS {
            let i = i as usize;
            &self.suggestions.items[i].display();
        }
    }

    pub fn update() {

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
