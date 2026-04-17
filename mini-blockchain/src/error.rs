#[derive(Debug)]
pub enum BlockchainError {
    EmptyChain,
    InvalidBlock(u64),
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainError::EmptyChain => write!(f, "Blockchain has no blocks"),
            BlockchainError::InvalidBlock(i) => write!(f, "Block {} failed integrity check", i),
        }
    }
}

impl std::error::Error for BlockchainError {}