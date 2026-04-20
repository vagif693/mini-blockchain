mod block;
mod blockchain;
mod error;

use blockchain::Blockchain;

fn main() {
    println!("🚀 Starting Mini Blockchain\n");

    let mut bc = Blockchain::new(2);

    bc.add_block(String::from("Alice sends 10 BTC to Bob")).unwrap();
    bc.add_block(String::from("Bob sends 3 BTC to Charlie")).unwrap();
    bc.add_block(String::from("Charlie sends 1 BTC to Alice")).unwrap();

    bc.print_chain();

    match bc.is_valid() {
        Ok(()) => println!("✅ Blockchain is valid!"),
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n🔧 Tampering with Block 1...\n");
    bc.chain[1].data = String::from("Alice sends 1000 BTC to Hacker");

    match bc.is_valid() {
        Ok(()) => println!("✅ Blockchain is valid!"),
        Err(e) => println!("❌ Tampering detected: {}", e),
    }
}