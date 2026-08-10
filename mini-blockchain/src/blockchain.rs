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

    /// Walk every block and verify three things:
    ///   1. The stored hash still matches a fresh recalculation (detects data edits)
    ///   2. The hash actually meets the difficulty target (detects unmined blocks)
    ///   3. The previous_hash field matches the actual previous block's hash (detects reordering)
    ///
    /// The genesis block is checked too — it is mined like any other block, so a
    /// tampered block 0 must not slip through just because nothing precedes it.
    ///
    /// Returns Ok(()) if the chain is intact, or the first error encountered.
    pub fn is_valid(&self) -> Result<(), BlockchainError> {
        if self.chain.is_empty() {
            return Err(BlockchainError::EmptyChain);
        }

        let target = "0".repeat(self.difficulty);

        for (i, current) in self.chain.iter().enumerate() {
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

            // Without this, a block carrying a perfectly consistent but never-mined
            // hash would validate, and the proof of work would be decorative.
            if !current.hash.starts_with(&target) {
                return Err(BlockchainError::InsufficientWork(current.index));
            }

            if i > 0 && current.previous_hash != self.chain[i - 1].hash {
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

#[cfg(test)]
mod tests {
    use super::*;

    const DIFFICULTY: usize = 2;

    fn target() -> String {
        "0".repeat(DIFFICULTY)
    }

    #[test]
    fn genesis_is_created_and_mined() {
        let chain = Blockchain::new(DIFFICULTY);
        let genesis = &chain.chain[0];

        assert_eq!(chain.chain.len(), 1);
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert!(genesis.hash.starts_with(&target()));
    }

    #[test]
    fn added_blocks_link_to_their_predecessor() {
        let mut chain = Blockchain::new(DIFFICULTY);
        chain.add_block(String::from("first")).unwrap();
        chain.add_block(String::from("second")).unwrap();

        assert_eq!(chain.chain.len(), 3);
        for i in 1..chain.chain.len() {
            assert_eq!(chain.chain[i].index, i as u64);
            assert_eq!(chain.chain[i].previous_hash, chain.chain[i - 1].hash);
        }
        assert!(chain.is_valid().is_ok());
    }

    #[test]
    fn tampering_with_data_is_detected() {
        let mut chain = Blockchain::new(DIFFICULTY);
        chain.add_block(String::from("Alice pays Bob")).unwrap();
        chain.chain[1].data = String::from("Alice pays the attacker");

        assert!(matches!(
            chain.is_valid(),
            Err(BlockchainError::InvalidHash(1))
        ));
    }

    #[test]
    fn tampering_with_the_genesis_block_is_detected() {
        let mut chain = Blockchain::new(DIFFICULTY);
        chain.add_block(String::from("first")).unwrap();
        chain.chain[0].data = String::from("rewritten history");

        assert!(matches!(
            chain.is_valid(),
            Err(BlockchainError::InvalidHash(0))
        ));
    }

    #[test]
    fn a_consistent_but_unmined_block_is_rejected() {
        let mut chain = Blockchain::new(DIFFICULTY);
        let previous_hash = chain.last_block().unwrap().hash.clone();

        // Internally honest — the stored hash really is the hash of these fields —
        // but nobody ever did the work. Nudge the nonce until the hash clearly
        // misses the target, so the test never rides on a lucky leading zero.
        let mut forged = Block::new(1, String::from("free money"), previous_hash);
        while forged.hash.starts_with(&target()) {
            forged.nonce += 1;
            forged.hash = Block::calculate_hash(
                forged.index,
                &forged.timestamp,
                &forged.data,
                &forged.previous_hash,
                forged.nonce,
            );
        }
        chain.chain.push(forged);

        assert!(matches!(
            chain.is_valid(),
            Err(BlockchainError::InsufficientWork(1))
        ));
    }

    #[test]
    fn a_rewritten_link_is_detected() {
        let mut chain = Blockchain::new(DIFFICULTY);
        chain.add_block(String::from("first")).unwrap();
        chain.add_block(String::from("second")).unwrap();

        // Re-point block 2 at the genesis block and re-mine it, so the hash is
        // valid and meets the target and the only remaining fault is the linkage.
        chain.chain[2].previous_hash = chain.chain[0].hash.clone();
        chain.chain[2].mine(DIFFICULTY);

        assert!(matches!(
            chain.is_valid(),
            Err(BlockchainError::BrokenLink(2))
        ));
    }

    #[test]
    fn an_empty_chain_reports_empty() {
        let mut chain = Blockchain::new(DIFFICULTY);
        chain.chain.clear();

        assert!(matches!(chain.last_block(), Err(BlockchainError::EmptyChain)));
        assert!(matches!(chain.is_valid(), Err(BlockchainError::EmptyChain)));
    }
}