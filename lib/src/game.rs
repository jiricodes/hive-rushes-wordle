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
    Grey,
    Yellow,
    Green,
}

impl LetterStatus {
    pub fn as_char(&self) -> char {
        match *self {
            Self::Green => 'G',
            Self::Grey => 'X',
            Self::Yellow => 'Y',
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            Self::Green => "G",
            Self::Grey => "X",
            Self::Yellow => "Y",
        }
    }
}

/// Custom Type, consider using struct instead
pub type WordStatus = Vec<LetterStatus>;

/// Helper function just to return the word status as `String`
pub fn status_as_string(status: &WordStatus) -> String {
    let mut out = String::new();
    for letterstatus in status {
        out += letterstatus.as_str();
    }
    out
}

pub fn status_green(status: &WordStatus) -> bool {
    for letterstatus in status {
        if *letterstatus != LetterStatus::Green {
            return false;
        }
    }
    true
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
        let mut status: WordStatus = Vec::new();
        for (position, letter) in word.chars().enumerate() {
            let c = self.word.chars().nth(position).unwrap();
            eprintln!("({}: {} | {})", position, letter, c);
            if letter == c {
                status.push(LetterStatus::Green);
            } else if self.word.contains(letter) {
                status.push(LetterStatus::Yellow);
            } else {
                status.push(LetterStatus::Grey);
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
