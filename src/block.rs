use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    index: u32,
    hash: [u8; 32],
    prev_hash: [u8; 32],
    tx: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Block {
        Block {index: 1234, hash: [3; 32], prev_hash: [4; 32], tx: Vec::new()}
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_hash(&self) -> &[u8; 32] {
        &self.hash
    }
}