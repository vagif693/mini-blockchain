use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::Utc;

// ============================================================
// BLOCK STRUCT
// A Block is one "page" in our blockchain "notebook"
// ============================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    index: u64,           // Which block is this? (0, 1, 2, 3...)
    timestamp: String,    // When was it created?
    data: String,         // What data/transaction does it hold?
    previous_hash: String,// The hash of the block BEFORE this one
    hash: String,         // This block's own unique fingerprint
    nonce: u64,           // A number we change during mining
}

impl Block {
    // ----------------------------------------------------------
    // Create a brand new block
    // ----------------------------------------------------------
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let nonce = 0;

        // Calculate the hash for this block right away
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash, nonce);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    // ----------------------------------------------------------
    // Calculate a SHA-256 hash from all the block's fields
    // Think of it like a "fingerprint" of everything in the block
    // ----------------------------------------------------------
    fn calculate_hash(
        index: u64,
        timestamp: &str,
        data: &str,
        previous_hash: &str,
        nonce: u64,
    ) -> String {
        // Mash all the fields together into one big string
        let input = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);

        // Feed that string into SHA-256
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        // Convert the raw bytes into a readable hex string like "a3f9bc..."
        hex::encode(result)
    }

    // ----------------------------------------------------------
    // MINING: Keep changing the nonce until the hash
    // starts with a certain number of zeros.
    // Example with difficulty=2: hash must start with "00"
    // ----------------------------------------------------------
    fn mine(&mut self, difficulty: usize) {
        // Build the target prefix: if difficulty=2, target = "00"
        let target = "0".repeat(difficulty);

        println!("⛏️  Mining block {}...", self.index);

        // Keep looping until we find a valid hash
        loop {
            self.hash = Block::calculate_hash(
                self.index,
                &self.timestamp,
                &self.data,
                &self.previous_hash,
                self.nonce,
            );

            // Does our hash start with enough zeros?
            if self.hash.starts_with(&target) {
                println!("✅ Block mined! Nonce: {} | Hash: {}", self.nonce, &self.hash[..12]);
                break;
            }

            // No? Try the next nonce
            self.nonce += 1;
        }
    }
}

// ============================================================
// BLOCKCHAIN STRUCT
// The blockchain is just a list (Vec) of Blocks
// ============================================================
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize, // How many leading zeros required when mining
}

impl Blockchain {
    // ----------------------------------------------------------
    // Create a new blockchain — automatically adds the Genesis block
    // The Genesis block is just the very first block (index 0)
    // It has no "previous block" so previous_hash = "0"
    // ----------------------------------------------------------
    fn new(difficulty: usize) -> Blockchain {
        let mut genesis = Block::new(0, String::from("Genesis Block"), String::from("0"));
        genesis.mine(difficulty);

        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    // ----------------------------------------------------------
    // Get the last block in the chain (we need its hash for the next block)
    // ----------------------------------------------------------
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // ----------------------------------------------------------
    // Add a new block to the chain
    // ----------------------------------------------------------
    fn add_block(&mut self, data: String) {
        let previous_hash = self.last_block().hash.clone();
        let index = self.chain.len() as u64;

        let mut new_block = Block::new(index, data, previous_hash);
        new_block.mine(self.difficulty);

        self.chain.push(new_block);
    }

    // ----------------------------------------------------------
    // VALIDATION: Check if the entire chain is legit
    // We loop through every block and check two things:
    //   1. Its hash is still correct (wasn't tampered with)
    //   2. Its previous_hash matches the actual previous block
    // ----------------------------------------------------------
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Recalculate what the hash SHOULD be
            let recalculated = Block::calculate_hash(
                current.index,
                &current.timestamp,
                &current.data,
                &current.previous_hash,
                current.nonce,
            );

            // If hashes don't match — someone tampered with the block!
            if current.hash != recalculated {
                println!("❌ Block {} has been tampered with!", current.index);
                return false;
            }

            // If the link to the previous block is broken
            if current.previous_hash != previous.hash {
                println!("❌ Block {} is not linked correctly!", current.index);
                return false;
            }
        }
        true
    }

    // ----------------------------------------------------------
    // Print the whole chain in a readable format
    // ----------------------------------------------------------
    fn print_chain(&self) {
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

// ============================================================
// MAIN — Where the program starts
// ============================================================
fn main() {
    println!("🚀 Starting Mini Blockchain\n");

    // Create a new blockchain with difficulty 2
    // (hashes must start with "00")
    let mut blockchain = Blockchain::new(2);

    // Add some blocks
    blockchain.add_block(String::from("Alice sends 10 BTC to Bob"));
    blockchain.add_block(String::from("Bob sends 3 BTC to Charlie"));
    blockchain.add_block(String::from("Charlie sends 1 BTC to Alice"));

    // Print the whole chain
    blockchain.print_chain();

    // Validate the chain
    if blockchain.is_valid() {
        println!("✅ Blockchain is valid!");
    } else {
        println!("❌ Blockchain is INVALID!");
    }

    // --- TAMPER DEMO ---
    // Let's manually change block 1's data and see if validation catches it
    println!("\n🔧 Tampering with Block 1...\n");
    blockchain.chain[1].data = String::from("Alice sends 1000 BTC to Hacker");

    if blockchain.is_valid() {
        println!("✅ Blockchain is valid!");
    } else {
        println!("❌ Blockchain is INVALID! Tampering detected.");
    }
}