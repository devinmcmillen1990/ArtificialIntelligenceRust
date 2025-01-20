use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

/// Cache for the standard minimax algorithm.
#[derive(Serialize, Deserialize)]
pub struct MinimaxCache {
    /// Stores the hash of the board state and its corresponding minimax evaluation score.
    pub map: HashMap<u64, i32>,
}

impl MinimaxCache {
    /// Creates a new empty `MinimaxCache`.
    pub fn new() -> Self {
        MinimaxCache { map: HashMap::new() }
    }

    /// Loads a `MinimaxCache` from a file.
    /// 
    /// # Parameters
    /// - `filename`: The path to the file where the cache is stored.
    /// 
    /// # Returns
    /// A `MinimaxCache` instance loaded from the file, or a new instance if the file cannot be read or parsed.
    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_else(|_| MinimaxCache::new())
        } else {
            MinimaxCache::new()
        }
    }

    /// Saves the `MinimaxCache` to a file.
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