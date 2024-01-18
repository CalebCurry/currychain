use std::collections::HashMap;

use bls_signatures::{PrivateKey, PublicKey};
use rand::thread_rng;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, u128>,
    pub pending_transactions: Vec<Transaction>,
}
#[derive(Debug)]
pub struct Block {
    pub transactions: Vec<Transaction>,
}
#[derive(Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

pub struct Key {
    pub private: PrivateKey,
    pub public: PublicKey,
}

impl Block {
    pub fn new() -> Self {
        Block {
            transactions: Vec::new(),
        }
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn create_block(&mut self) {
        let mut block = Block::new();
        for txn in self.pending_transactions.drain(..) {
            println!("txn: {txn:?}");
            //update acct balance

            //genesis transactions don't subtract
            if self.blocks.len() != 0 {
                let current_from = self.accounts.entry(txn.from.clone()).or_insert(0);
                *current_from -= txn.amount;
            }

            let current = self.accounts.entry(txn.to.clone()).or_insert(0);
            *current += txn.amount;

            block.transactions.push(txn);
        }
        self.blocks.push(block);
    }
}

impl Key {
    pub fn new() -> Key {
        let key = bls_signatures::PrivateKey::generate(&mut thread_rng());

        Key {
            private: key,
            public: key.public_key(),
        }
    }
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u128) -> Transaction {
        Transaction { amount, from, to }
    }
    pub fn spend(self, blockchain: &mut Blockchain) {
        //check if valid spend

        if blockchain.blocks.len() != 0 {
            if let Some(from) = blockchain.accounts.get(&self.from) {
                if from < &self.amount {
                    return;
                }
            }
        }
        blockchain.pending_transactions.push(self);
    }
}
