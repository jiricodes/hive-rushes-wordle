//! Word database module
//!

use indexmap::IndexSet;
use rand::{thread_rng, Rng};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Main database struct
///
/// Using IndexSet as opposed to HashSet. The [IndexSet](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html) allows us to
/// efficiently random sample the words.
///
/// WIP
/// perhaps we dont need discarded, however lets keep if for now
#[derive(Debug)]
pub struct Database {
    available: IndexSet<String>,
    discarded: IndexSet<String>,
}

impl Database {
    /// Simple constructor, returns default aka empty
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the database from a file
    ///
    /// Panics on file opening error or line reading error
    ///
    /// # Example
    /// ```
    /// use lib::database::Database;
    ///
    /// let d = Database::load("../data/test_dict.txt");
    /// dbg!(d);
    /// ```
    pub fn load<P>(filename: P) -> Self
    where
        P: AsRef<Path> + Debug,
    {
        let file = File::open(filename).expect("File error");
        let reader = BufReader::new(file);
        let mut all_words: IndexSet<String> = IndexSet::new();
        for line in reader.lines() {
            all_words.insert(line.expect("line error"));
        }
        Self {
            available: all_words,
            ..Default::default()
        }
    }

    /// Returns random word from the available set
    /// Does not consume.
    pub fn get_random(&self) -> String {
        let mut rng = thread_rng();
        let i = rng.gen_range(0..self.available.len()) as usize;
        self.available.get_index(i).unwrap().clone()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            available: IndexSet::new(),
            discarded: IndexSet::new(),
        }
    }
}
