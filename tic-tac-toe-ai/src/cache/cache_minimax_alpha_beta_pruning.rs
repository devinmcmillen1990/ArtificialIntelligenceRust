use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

/// Cache for the alpha-beta pruning minimax algorithm.
#[derive(Serialize, Deserialize)]
pub struct AlphaBetaCache {
    /// Stores the hash of the board state and its corresponding alpha-beta evaluation score.
    pub map: HashMap<u64, i32>,
}

impl AlphaBetaCache {
    /// Creates a new empty `AlphaBetaCache`.
    pub fn new() -> Self {
        AlphaBetaCache { map: HashMap::new() }
    }

    /// Loads an `AlphaBetaCache` from a file.
    /// 
    /// # Parameters
    /// - `filename`: The path to the file where the cache is stored.
    /// 
    /// # Returns
    /// An `AlphaBetaCache` instance loaded from the file, or a new instance if the file cannot be read or parsed.
    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_default();
            serde_json::from_str(&contents).unwrap_or_else(|_| AlphaBetaCache::new())
        } else {
            AlphaBetaCache::new()
        }
    }

    /// Saves the `AlphaBetaCache` to a file.
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