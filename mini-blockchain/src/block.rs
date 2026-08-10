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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_hash_is_deterministic() {
        let first = Block::calculate_hash(1, "2026-01-01T00:00:00Z", "data", "prev", 42);
        let second = Block::calculate_hash(1, "2026-01-01T00:00:00Z", "data", "prev", 42);

        assert_eq!(first, second);
    }

    #[test]
    fn hash_is_64_hex_characters() {
        let hash = Block::calculate_hash(0, "t", "d", "p", 0);

        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn changing_any_field_changes_the_hash() {
        let base = Block::calculate_hash(1, "t", "d", "p", 0);

        assert_ne!(base, Block::calculate_hash(2, "t", "d", "p", 0));
        assert_ne!(base, Block::calculate_hash(1, "u", "d", "p", 0));
        assert_ne!(base, Block::calculate_hash(1, "t", "e", "p", 0));
        assert_ne!(base, Block::calculate_hash(1, "t", "d", "q", 0));
        assert_ne!(base, Block::calculate_hash(1, "t", "d", "p", 1));
    }

    #[test]
    fn new_block_records_its_inputs() {
        let block = Block::new(7, String::from("payload"), String::from("abc"));

        assert_eq!(block.index, 7);
        assert_eq!(block.data, "payload");
        assert_eq!(block.previous_hash, "abc");
        assert_eq!(block.nonce, 0);
    }

    #[test]
    fn mining_hits_the_target_and_leaves_the_hash_consistent() {
        let mut block = Block::new(1, String::from("payload"), String::from("abc"));
        block.mine(3);

        assert!(block.hash.starts_with("000"));
        assert_eq!(
            block.hash,
            Block::calculate_hash(
                block.index,
                &block.timestamp,
                &block.data,
                &block.previous_hash,
                block.nonce,
            )
        );
    }
}