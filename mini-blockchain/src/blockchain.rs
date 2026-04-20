use crate::block::Block;
use crate::error::BlockchainError;

/// A Blockchain is an ordered list of Blocks where each block
/// cryptographically references the one before it.
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Create a new chain. The genesis block (index 0) is automatically
    /// created and mined. It has no real previous block, so previous_hash = "0".
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, String::from("Genesis Block"), String::from("0"));
        genesis.mine(difficulty);

        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    /// Returns a reference to the most recent block in the chain.
    /// Returns an error if the chain is somehow empty.
    pub fn last_block(&self) -> Result<&Block, BlockchainError> {
        self.chain.last().ok_or(BlockchainError::EmptyChain)
    }

    /// Mine a new block with the given data and append it to the chain.
    /// The new block's previous_hash is set to the current last block's hash.
    pub fn add_block(&mut self, data: String) -> Result<(), BlockchainError> {
        let previous_hash = self.last_block()?.hash.clone();
        let index = self.chain.len() as u64;

        let mut block = Block::new(index, data, previous_hash);
        block.mine(self.difficulty);
        self.chain.push(block);

        Ok(())
    }

    /// Walk every block and verify two things:
    ///   1. The stored hash still matches a fresh recalculation (detects data edits)
    ///   2. The previous_hash field matches the actual previous block's hash (detects reordering)
    ///
    /// Returns Ok(()) if the chain is intact, or the first error encountered.
    pub fn is_valid(&self) -> Result<(), BlockchainError> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated = Block::calculate_hash(
                current.index,
                &current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );

            if current.hash != recalculated {
                return Err(BlockchainError::InvalidHash(current.index));
            }

            if current.previous_hash != previous.hash {
                return Err(BlockchainError::BrokenLink(current.index));
            }
        }

        Ok(())
    }

    /// Print every block in a readable format.
    pub fn print_chain(&self) {
        println!("\n========== 🔗 BLOCKCHAIN ==========");
        for block in &self.chain {
            let prev_display = if block.previous_hash.len() >= 12 {
                format!("{}...", &block.previous_hash[..12])
            } else {
                block.previous_hash.clone()
            };

            println!("\n📦 Block #{}", block.index);
            println!("   Data     : {}", block.data);
            println!("   Nonce    : {}", block.nonce);
            println!("   Prev Hash: {}", prev_display);
            println!("   Hash     : {}...", &block.hash[..12]);
            println!("   Time     : {}", block.timestamp);
        }
        println!("\n====================================\n");
    }
}