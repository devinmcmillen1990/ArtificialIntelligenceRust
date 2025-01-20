use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

/// Represents the evaluation of a state in the weighted minimax algorithm.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeightedStateEvaluation {
    /// The minimax evaluation score of the state.
    pub score: i32,
    /// The weights associated with the board positions for the state.
    pub weights: Vec<i32>,
}

/// Cache for the weighted minimax algorithm.
#[derive(Serialize, Deserialize)]
pub struct WeightedCache {
    /// Stores the hash of the board state and its corresponding weighted evaluation.
    pub map: HashMap<u64, WeightedStateEvaluation>,
}

impl WeightedCache {
    /// Creates a new empty `WeightedCache`.
    pub fn new() -> Self {
        WeightedCache { map: HashMap::new() }
    }

    /// Loads a `WeightedCache` from a file.
    ///
    /// # Parameters
    /// - `filename`: The path to the file where the cache is stored.
    ///
    /// # Returns
    /// A `WeightedCache` instance loaded from the file, or a new instance if the file cannot be read or parsed.
    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_else(|_| WeightedCache::new())
        } else {
            WeightedCache::new()
        }
    }

    /// Saves the `WeightedCache` to a file.
    ///
    /// # Parameters
    /// - `filename`: The path to the file where the cache should be saved.
    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string(self).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
