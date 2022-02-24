use std::collections::HashMap;

pub struct LetterFrequencyMap {
    pub map: HashMap<char, f32>,
}

/// Letter frequencies are from https://www3.nd.edu/~busiforc/handouts/cryptography/letterfrequencies.html
impl LetterFrequencyMap (
    /// Constructor
    pub fn new() -> Self {
        let map = HashMap::from([
            ("a", 43.31),
            ("b", 10.56),
            ("c", 23.13),
            ("d", 17.25),
            ("e", 56.88),
            ("f", 9.24),
            ("g", 12.59),
            ("h", 15.31),
            ("i", 38.45),
            ("j", 1.00),
            ("k", 5.61),
            ("l", 27.98),
            ("m", 15.36),
            ("n", 33.92),
            ("o", 36.51),
            ("p", 16.14),
            ("q", 1.0),
            ("r", 38.64),
            ("s", 29.23),
            ("t", 35.43),
            ("u", 18.51),
            ("v", 5.13),
            ("w", 6.57),
            ("x", 1.48),
            ("y", 9.06),
            ("z", 1.39),
]);
        map
    }
}
