use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

/// Cache for standard minimax algorithm
#[derive(Serialize, Deserialize)]
pub struct MinimaxCache {
    pub map: HashMap<u64, i32>,
}

impl MinimaxCache {
    pub fn new() -> Self {
        MinimaxCache { map: HashMap::new() }
    }

    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap_or_else(|_| MinimaxCache::new())
        } else {
            MinimaxCache::new()
        }
    }

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

/// Cache for weighted minimax algorithm
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeightedStateEvaluation {
    pub score: i32,
    pub weights: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct WeightedCache {
    pub map: HashMap<u64, WeightedStateEvaluation>,
}

impl WeightedCache {
    pub fn new() -> Self {
        WeightedCache { map: HashMap::new() }
    }

    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = File::open(filename) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap_or_else(|_| WeightedCache::new())
        } else {
            WeightedCache::new()
        }
    }

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
