//! Game of Wordle module
//!

use std::fmt;
use std::fmt::Display;

/// Enum to show a letter status
/// Grey - letter not in word
/// Yellow - letter in word
/// Greem - letter at correct position
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    pub fn from_chars(c: char, status: char) -> Option<Self> {
        match status {
            'G' => Some(Self::Green(c)),
            'X' => Some(Self::Grey(c)),
            'Y' => Some(Self::Yellow(c)),
            _ => None,
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

    pub fn from_strings(input: &String, status: &String) -> Self {
        assert!(
            input.len() == status.len(),
            "Input and status strings are different lenght"
        );
        let s = status.to_uppercase();
        let mut ret = Self::default();
        for (p, c) in input.chars().enumerate() {
            let sc = s.chars().nth(p).unwrap();
            ret.push(LetterStatus::from_chars(c, sc).expect("Unknown char in status string"));
        }
        ret
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

    pub fn push_front(&mut self, letter_status: LetterStatus) {
        self.data.insert(0, letter_status)
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

    /// tuple (green, yellow, greey)
    pub fn char_count(&self, c: char) -> (usize, usize, usize) {
        let mut count: (usize, usize, usize) = (0, 0, 0);
        for status in self.data.iter() {
            match status {
                LetterStatus::Green(val) => {
                    if *val == c {
                        count.0 += 1;
                    }
                }
                LetterStatus::Grey(val) => {
                    if *val == c {
                        count.2 += 1;
                    }
                }
                LetterStatus::Yellow(val) => {
                    if *val == c {
                        count.1 += 1;
                    }
                }
            }
        }
        count
    }

    pub fn get_green_chars(&self) -> String {
        let mut ret = String::new();
        for val in self.data.iter() {
            match val {
                LetterStatus::Green(c) => ret.push(*c),
                _ => {}
            }
        }
        ret
    }

    pub fn get_yellow_chars(&self) -> String {
        let mut ret = String::new();
        for val in self.data.iter() {
            match val {
                LetterStatus::Yellow(c) => ret.push(*c),
                _ => {}
            }
        }
        ret
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

        // Check for letters multiple instances
        let mut result = WordStatus::new();
        for letter_status in status.iter() {
            match letter_status {
                // We care only about yellows - greens are set and grey are as low as they can be
                LetterStatus::Yellow(letter) => {
                    let cnt_org = self.word.matches(*letter).count();
                    let cnt_guess = word.matches(*letter).count();
                    // if there's more instances than in original word
                    if cnt_guess > cnt_org {
                        // we check what kind of statuses they have
                        // (green, yellow, grey)
                        let cnt_status = status.char_count(*letter);
                        let cnt_res = result.char_count(*letter);
                        if cnt_status.0 + cnt_res.1 >= cnt_org {
                            result.push(LetterStatus::Grey(*letter))
                        } else {
                            result.push(*letter_status)
                        }
                    } else {
                        result.push(*letter_status);
                    }
                }
                _ => {
                    result.push(*letter_status);
                }
            }
        }
        // Sanity check, There should be same number of elements in `status`
        // as is in the word
        assert!(
            result.len() == word.len(),
            "WordStatus as different length than the original word"
        );
        self.attempts += 1;
        result
    }
}
