//! A blockchain implementation with proof-of-work mining.
///
/// This module contains structures and functionality for a simple blockchain,
/// including transaction management, block creation, and proof-of-work mining.
use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};

/// Represents a transaction between two parties.
///
/// A transaction records the transfer of assets from a sender to a receiver.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Transaction {
    /// Address of the sender
    pub sender: String,
    /// Address of the receiver
    pub receiver: String,
    /// Amount transferred
    pub amount: f32,
}

/// Header information for a block in the blockchain.
///
/// Contains metadata and proof-of-work elements required for blockchain integrity.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct BlockHeader {
    /// Time when the block was created
    timestamp: DateTime<Utc>,
    /// Counter used for proof-of-work mining
    nonce: u64,
    /// Hash of the previous block in the chain
    previous_hash: String,
    /// Merkle root of all transactions in this block
    merkle: String,
    /// Number of leading zeros required in hash (mining difficulty)
    difficulty: u32,
}

impl BlockHeader {
    /// Returns the nonce value of this block header.
    pub(crate) const fn get_nonce(&self) -> u64 {
        self.nonce
    }

    /// Returns the hash of the previous block in the chain.
    pub(crate) fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }
}

/// A block in the blockchain containing transactions.
///
/// Each block includes a header with metadata and a list of transactions.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct Block {
    /// Metadata and proof-of-work information
    header: BlockHeader,
    /// Number of transactions in this block
    count: u32,
    /// List of transactions included in this block
    transactions: Vec<Transaction>,
}

impl Block {
    /// Returns a reference to the block header.
    pub(crate) const fn get_header(&self) -> &BlockHeader {
        &self.header
    }

    /// Returns a reference to the transactions in this block.
    pub(crate) const fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

/// The main blockchain data structure.
///
/// Manages the chain of blocks, pending transactions, and mining operations.
pub(crate) struct Chain {
    /// The sequence of validated blocks forming the blockchain
    chains: Vec<Block>,
    /// Pending transactions awaiting inclusion in the next block
    current_transactions: Vec<Transaction>,
    /// Mining difficulty (number of leading zeros required in block hash)
    difficulty: u32,
    /// Address where mining rewards should be sent
    miner_address: String,
    /// Amount awarded to the miner for successfully mining a block
    reward: f32,
}

impl Chain {
    /// Creates a new blockchain with a genesis block.
    ///
    /// # Arguments
    ///
    /// * `miner_address` - Address where mining rewards will be sent
    /// * `difficulty` - Initial mining difficulty (number of leading zeros required in hash)
    /// * `reward` - Optional mining reward amount (defaults to 50.0 if None)
    ///
    /// # Returns
    ///
    /// A new Chain instance with a genesis block
    pub(crate) fn new(miner_address: String, difficulty: u32, reward: Option<f32>) -> Self {
        let reward = reward.unwrap_or(50.0); // Default reward if not provided
        let mut chain = Self {
            chains: Vec::new(),
            current_transactions: Vec::new(),
            difficulty,
            miner_address,
            reward,
        };
        chain.generate_new_block();
        chain
    }

    /// Adds a new transaction to the pending transaction pool.
    ///
    /// # Arguments
    ///
    /// * `sender` - Address of the transaction sender
    /// * `receiver` - Address of the transaction receiver
    /// * `amount` - Amount to transfer
    ///
    /// # Returns
    ///
    /// `true` if the transaction was successfully added
    pub(crate) fn add_transaction(
        &mut self,
        sender: String,
        receiver: String,
        amount: f32,
    ) -> bool {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.current_transactions.push(transaction);
        true
    }

    /// Computes the SHA-256 hash of a serializable item.
    ///
    /// # Arguments
    ///
    /// * `item` - Any serializable item to hash
    ///
    /// # Returns
    ///
    /// Hexadecimal string representation of the hash
    pub(crate) fn hash<T: Serialize>(item: &T) -> String {
        let update = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(update.as_bytes());
        let result = hasher.finalize();
        let vec_res = result.to_vec();

        Self::hex_to_string(vec_res.as_slice())
    }

    /// Converts a byte slice to a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `vec_res` - Slice of bytes to convert
    ///
    /// # Returns
    ///
    /// Hexadecimal string representation of the bytes
    pub(crate) fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::with_capacity(vec_res.len() * 2);
        for b in vec_res {
            use std::fmt::Write;
            write!(&mut s, "{b:02x}").unwrap();
        }
        s
    }

    /// Gets the hash of the last block in the chain.
    ///
    /// # Returns
    ///
    /// Hash of the last block, or a string of zeros if the chain is empty
    pub(crate) fn last_hash(&self) -> String {
        let Some(block) = self.chains.last() else {
            return String::from_utf8(vec![48; 64]).unwrap();
        };
        Self::hash(&block.header)
    }

    /// Updates the mining difficulty.
    ///
    /// # Arguments
    ///
    /// * `new_difficulty` - New mining difficulty value
    ///
    /// # Returns
    ///
    /// `true` if the difficulty was successfully updated
    pub(crate) const fn update_difficulty(&mut self, new_difficulty: u32) -> bool {
        self.difficulty = new_difficulty;
        true
    }

    /// Updates the mining reward amount.
    ///
    /// # Arguments
    ///
    /// * `new_reward` - New mining reward value
    ///
    /// # Returns
    ///
    /// `true` if the reward was successfully updated
    pub(crate) const fn update_reward(&mut self, new_reward: f32) -> bool {
        self.reward = new_reward;
        true
    }

    /// Creates and mines a new block containing pending transactions.
    ///
    /// Includes a mining reward transaction and performs proof-of-work.
    ///
    /// # Returns
    ///
    /// `true` if the block was successfully generated and added to the chain
    pub(crate) fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: Utc::now(),
            nonce: 0,
            previous_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_transaction = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: Vec::new(),
        };

        block.transactions.push(reward_transaction);
        block.transactions.append(&mut self.current_transactions);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Self::get_merkle(&block.transactions.clone());
        Self::proof_of_work(&mut block.header);

        println!("Last {:#?}", &block);
        self.chains.push(block);
        true
    }

    /// Calculates the Merkle root of a set of transactions.
    ///
    /// # Arguments
    ///
    /// * `transactions` - List of transactions to include in the Merkle tree
    ///
    /// # Returns
    ///
    /// Merkle root hash as a string
    pub(crate) fn get_merkle(transactions: &[Transaction]) -> String {
        let mut merkle = Vec::new();

        for t in transactions {
            let hash = Self::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let h2 = merkle.remove(0);
            h1.push_str(&h2);
            let hn = Self::hash(&h1);
            merkle.push(hn);
        }

        merkle.pop().unwrap()
    }

    /// Performs proof-of-work mining on a block header.
    ///
    /// Repeatedly hashes the header with different nonce values until
    /// finding a hash with the required number of leading zeros.
    ///
    /// # Arguments
    ///
    /// * `header` - Block header to mine
    pub(crate) fn proof_of_work(header: &mut BlockHeader) {
        let difficulty = u64::from(header.difficulty);
        let pb = indicatif::ProgressBar::new(100);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .unwrap(),
        );
        let delta = 8 / difficulty;
        let handle = std::thread::spawn(move || {
            for _ in 0..(1024 / (delta)) {
                pb.inc(delta);
                std::thread::sleep(std::time::Duration::from_millis(difficulty * 10));
            }
            pb.finish_with_message("Mining complete!");
        });
        let m;
        loop {
            let hash = Self::hash(&header);
            let slice = &hash[..header.difficulty as usize];
            if let Ok(val) = slice.parse::<u32>() {
                if val != 0 {
                    header.nonce += 1;
                } else {
                    m = hash;
                    break;
                }
            } else {
                header.nonce += 1;
            }
        }
        handle.join().unwrap();
        println!("Block hashed: {m}");
    }

    /// Returns the JSON representation of the latest block.
    ///
    /// # Returns
    ///
    /// Pretty-printed JSON string of the latest block, or None if the chain is empty
    pub(crate) fn get_latest_block_json(&self) -> Option<String> {
        self.chains
            .last()
            .map(|b| serde_json::to_string_pretty(b).unwrap())
    }

    /// Returns JSON representations of all blocks in the chain.
    ///
    /// # Returns
    ///
    /// Vector of pretty-printed JSON strings for each block
    pub(crate) fn get_blocks_json(&self) -> Vec<String> {
        self.chains
            .iter()
            .map(|b| serde_json::to_string_pretty(b).unwrap())
            .collect()
    }

    /// Returns the current mining difficulty.
    pub(crate) const fn get_difficulty(&self) -> u32 {
        self.difficulty
    }

    /// Returns the current mining reward.
    pub(crate) const fn get_reward(&self) -> f32 {
        self.reward
    }

    /// Returns a reference to the blockchain.
    pub(crate) const fn get_chain(&self) -> &Vec<Block> {
        &self.chains
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chain_has_genesis_block() {
        let chain = Chain::new("Tilt".to_string(), 1, None);
        assert_eq!(chain.get_chain().len(), 1);
    }

    #[test]
    fn new_chain_uses_default_reward_when_not_provided() {
        let chain = Chain::new("Tilt".to_string(), 1, None);
        assert!((chain.get_reward() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn new_chain_uses_custom_reward_when_provided() {
        let chain = Chain::new("Tilt".to_string(), 1, Some(100.0));
        assert!((chain.get_reward() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn transaction_added_successfully() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        let result = chain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0);
        assert!(result);
        assert_eq!(chain.current_transactions.len(), 1);
    }

    #[test]
    fn block_generation_includes_pending_transactions() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        chain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0);
        chain.add_transaction("Bob".to_string(), "Alice".to_string(), 20.0);

        let initial_chain_len = chain.get_chain().len();
        chain.generate_new_block();

        // The new block should contain the two transactions plus the reward transaction
        assert_eq!(chain.get_chain().len(), initial_chain_len + 1);
        assert_eq!(chain.get_chain().last().unwrap().count, 3);
        // Current transactions should be cleared
        assert_eq!(chain.current_transactions.len(), 0);
    }

    #[test]
    fn hash_produces_consistent_output_for_same_input() {
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 10.0,
        };

        let hash1 = Chain::hash(&transaction);
        let hash2 = Chain::hash(&transaction);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn hex_to_string_converts_bytes_to_hex() {
        let bytes = vec![0, 1, 10, 255];
        let hex = Chain::hex_to_string(&bytes);
        assert_eq!(hex, "00010aff");
    }

    #[test]
    fn last_hash_returns_hash_of_last_block() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        let last_hash = chain.last_hash();

        // Generate a new block and check that last_hash changes
        chain.generate_new_block();
        let new_last_hash = chain.last_hash();

        assert_ne!(last_hash, new_last_hash);
    }

    #[test]
    fn update_difficulty_changes_chain_difficulty() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        chain.update_difficulty(2);
        assert_eq!(chain.get_difficulty(), 2);
    }

    #[test]
    fn update_reward_changes_miner_reward() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        chain.update_reward(75.0);
        assert!((chain.get_reward() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn get_merkle_handles_odd_number_of_transactions() {
        let transactions = vec![
            Transaction {
                sender: "a".to_string(),
                receiver: "b".to_string(),
                amount: 1.0,
            },
            Transaction {
                sender: "c".to_string(),
                receiver: "d".to_string(),
                amount: 2.0,
            },
            Transaction {
                sender: "e".to_string(),
                receiver: "f".to_string(),
                amount: 3.0,
            },
        ];

        let merkle = Chain::get_merkle(&transactions);
        assert!(!merkle.is_empty());
    }

    #[test]
    fn get_latest_block_json_returns_none_for_empty_chain() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        chain.chains.clear(); // Artificially clear the chain
        assert!(chain.get_latest_block_json().is_none());
    }

    #[test]
    fn get_blocks_json_returns_expected_number_of_blocks() {
        let mut chain = Chain::new("Tilt".to_string(), 1, None);
        chain.generate_new_block();
        chain.generate_new_block();

        let blocks_json = chain.get_blocks_json();
        assert_eq!(blocks_json.len(), 3); // Genesis + 2 new blocks
    }
}
