use sha2::{Digest, Sha256};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Represents a transaction between two users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

/// Represents a block in the blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Creates and mines a new block.
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;
        let mut hash;
        
        loop {
            let to_hash = format!("{index}{timestamp}{:?}{previous_hash}{nonce}", transactions);
            let mut hasher = Sha256::new();
            hasher.update(to_hash);
            let result = hasher.finalize();
            hash = format!("{:x}", result);

            if hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }

            nonce += 1;
        }

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce,
            hash,
        }
    }
}

/// Represents the entire blockchain.
#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        if Path::new("blockchain.json").exists() {
            let data = fs::read_to_string("blockchain.json").expect("Failed to read file.");
            serde_json::from_str(&data).expect("Failed to parse blockchain.")
        } else {
            let genesis_block = Block::new(0, vec![], "0".to_string(), difficulty);
            Blockchain {
                chain: vec![genesis_block],
                difficulty,
            }
        }
    }

    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block = self.latest_block();
        let new_block = Block::new(
            last_block.index + 1,
            transactions,
            last_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
        self.save();
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            let to_hash = format!(
                "{}{}{:?}{}{}",
                current.index,
                current.timestamp,
                current.transactions,
                current.previous_hash,
                current.nonce
            );
            let mut hasher = Sha256::new();
            hasher.update(to_hash);
            let result = hasher.finalize();
            let recalculated_hash = format!("{:x}", result);

            if current.hash != recalculated_hash || current.previous_hash != previous.hash {
                return false;
            }

            if !current.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        true
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Serialization failed.");
        fs::write("blockchain.json", json).expect("Failed to save blockchain.");
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4);

    println!("Blockchain loaded. Current length: {}", blockchain.chain.len());

    loop {
        println!("\n--- New Transaction ---");
        print!("From: ");
        io::stdout().flush().unwrap();
        let from = read_input();

        print!("To: ");
        io::stdout().flush().unwrap();
        let to = read_input();

        print!("Amount: ");
        io::stdout().flush().unwrap();
        let amount_input = read_input();
        let amount: f64 = match amount_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid amount. Try again.");
                continue;
            }
        };

        let tx = Transaction { from, to, amount };
        blockchain.add_block(vec![tx]);
        println!("Block added and mined.");

        println!("\nBlockchain valid? {}", blockchain.is_valid());

        print!("Do you want to add another transaction? (y/n): ");
        io::stdout().flush().unwrap();
        let again = read_input();
        if again.to_lowercase().trim() != "y" {
            break;
        }
    }

    println!("\nFinal Blockchain:");
    for block in &blockchain.chain {
        println!("{:#?}", block);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
