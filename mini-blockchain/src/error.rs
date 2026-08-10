use std::fmt;

#[derive(Debug)]
pub enum BlockchainError {
    EmptyChain,
    InvalidHash(u64),
    BrokenLink(u64),
    InsufficientWork(u64),
}

impl fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockchainError::EmptyChain => {
                write!(f, "Blockchain has no blocks")
            }
            BlockchainError::InvalidHash(index) => {
                write!(f, "Block {} has an invalid hash — data may have been tampered with", index)
            }
            BlockchainError::BrokenLink(index) => {
                write!(f, "Block {} is not linked correctly to the previous block", index)
            }
            BlockchainError::InsufficientWork(index) => {
                write!(f, "Block {} does not meet the required proof-of-work difficulty", index)
            }
        }
    }
}

impl std::error::Error for BlockchainError {}