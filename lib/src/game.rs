//! Game of Wordle module
//!

use std::fmt;
use std::fmt::Display;

/// Enum to show a letter status
/// Grey - letter not in word
/// Yellow - letter in word
/// Greem - letter at correct position
#[derive(Debug, PartialEq, Eq)]
pub enum LetterStatus {
    Grey(char),
    Yellow(char),
    Green(char),
}

impl LetterStatus {
    pub fn as_char(&self) -> char {
        match *self {
            Self::Green(_) => 'G',
            Self::Grey(_) => 'X',
            Self::Yellow(_) => 'Y',
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            Self::Green(_) => "G",
            Self::Grey(_) => "X",
            Self::Yellow(_) => "Y",
        }
    }

    pub fn is_green(&self) -> bool {
        match *self {
            LetterStatus::Green(_) => true,
            _ => false,
        }
    }
}

/// Struct to handle guessed word status
pub struct WordStatus {
    data: Vec<LetterStatus>,
}

impl WordStatus {
    /// Constructor
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper function just to return the word status as `String`
    pub fn as_string(&self) -> String {
        let mut out = String::new();
        for letterstatus in self.data.iter() {
            out += letterstatus.as_str();
        }
        out
    }

    /// Checks if all letters have `LetterStatus::Green`
    pub fn is_correct(&self) -> bool {
        for letterstatus in self.data.iter() {
            if !letterstatus.is_green() {
                return false;
            }
        }
        true
    }

    /// Pushes LetterStatus to data
    pub fn push(&mut self, letter_status: LetterStatus) {
        self.data.push(letter_status)
    }

    /// Checks if data contains specific `LetterStatus`
    pub fn contains(&self, letter_status: &LetterStatus) -> bool {
        self.data.contains(letter_status)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, LetterStatus> {
        self.data.iter()
    }
}

impl Default for WordStatus {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

pub struct Wordle {
    word: String,
    max_attempts: usize,
    attempts: usize,
}

impl Default for Wordle {
    fn default() -> Self {
        Self {
            word: "hello".to_string(),
            max_attempts: 6,
            attempts: 0,
        }
    }
}

impl Wordle {
    /// Basic constructor
    pub fn new(word: String) -> Self {
        Self {
            word,
            ..Default::default()
        }
    }

    /// Max attempts setter
    pub fn set_max_attempts(&mut self, limit: usize) {
        self.max_attempts = limit;
    }

    /// Max attempts getter
    pub fn get_max_attempts(&self) -> usize {
        self.max_attempts
    }

    /// Check if game is over
    ///
    /// Returns bool if `attempts` >= `max_attempts`
    pub fn game_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }

    /// Checks guessed word against the hidden one. Outputs "color code" per letter
    /// as described in rules.
    pub fn guess_word(&mut self, word: &String) -> WordStatus {
        // Sanity check, guessed word must be same length
        assert!(
            self.word.len() == word.len(),
            "Guessed word is incorrect length"
        );
        // Return vector
        let mut status = WordStatus::new();
        for (position, letter) in word.chars().enumerate() {
            let c = self.word.chars().nth(position).unwrap();
            // eprintln!("({}: {} | {})", position, letter, c);
            if letter == c {
                status.push(LetterStatus::Green(letter));
            } else if self.word.contains(letter) {
                status.push(LetterStatus::Yellow(letter));
            } else {
                status.push(LetterStatus::Grey(letter));
            }
        }
        // Sanity check, There should be same number of elements in `status`
        // as is in the word
        assert!(
            status.len() == word.len(),
            "WordStatus as different length than the original word"
        );
        self.attempts += 1;
        status
    }
}
