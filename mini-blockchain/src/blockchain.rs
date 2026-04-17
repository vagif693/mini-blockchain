use crate::block::Block;
use crate::error::BlockchainError;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, String::from("Genesis Block"), String::from("0"));
        genesis.mine(difficulty);
        Blockchain { chain: vec![genesis], difficulty }
    }

    pub fn last_block(&self) -> Result<&Block, BlockchainError> {
        self.chain.last().ok_or(BlockchainError::EmptyChain)
    }

    pub fn add_block(&mut self, data: String) -> Result<(), BlockchainError> {
        let previous_hash = self.last_block()?.hash.clone();
        let index = self.chain.len() as u64;
        let mut block = Block::new(index, data, previous_hash);
        block.mine(self.difficulty);
        self.chain.push(block);
        Ok(())
    }

    pub fn is_valid(&self) -> Result<(), BlockchainError> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let recalculated = Block::calculate_hash(
                current.index, &current.timestamp,
                &current.data, &current.previous_hash, current.nonce,
            );

            if current.hash != recalculated {
                return Err(BlockchainError::InvalidBlock(current.index));
            }

            if current.previous_hash != previous.hash {
                return Err(BlockchainError::InvalidBlock(current.index));
            }
        }
        Ok(())
    }
}