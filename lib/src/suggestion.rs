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
        self.items.sort_by(|a, b| b.unique_chars.cmp(&a.unique_chars));
    }
}

impl From<&Vec<String>> for SuggestionCollection {
    fn from(string_vector: &Vec<String>) -> Self {
        let mut suggestion_collection = SuggestionCollection::new();
        for word in string_vector.iter() {
            let mut suggestion = Suggestion::new();
            suggestion.word = word.clone();
            suggestion_collection.items.push(suggestion);
        }
        suggestion_collection
    }
}

impl Default for SuggestionCollection {
    fn default() -> Self {
        Self {
            items: Vec::new()
        }
    }
}

pub struct Suggestion {
    word: String,
    probability: f32,
    unique_chars: i8,
}

impl Suggestion {

    /// Constructor
    pub fn new() -> Self {
        Self::default()
    }

    /// Display word
    pub fn display(&self) {
        println!("{} {}", &self.word, &self.probability);
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
