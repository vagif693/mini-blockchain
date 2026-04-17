use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let nonce = 0;
        let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);

        Block { index, timestamp, data, previous_hash, hash, nonce }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &str,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!("⛏️  Mining block {}...", self.index);

        loop {
            self.hash = Self::calculate_hash(
                self.index, &self.timestamp, &self.data, &self.previous_hash, self.nonce,
            );

            if self.hash.starts_with(&target) {
                println!("✅ Block mined! Nonce: {} | Hash: {}", self.nonce, &self.hash[..12]);
                break;
            }

            self.nonce += 1;
        }
    }
}