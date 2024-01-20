use base58::ToBase58;
use bls_signatures::{PrivateKey, PublicKey, Serialize, Signature};
use rand::thread_rng;
use serde;
use sha256;
use std::collections::HashMap;

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
#[derive(Debug, serde::Serialize)]
pub struct Transaction {
    pub from: String,
    pub signature: String,
    pub to: String,
    pub sender_public_key: String,
    pub hash: String,
    pub amount: u128,
}

pub struct Key {
    pub private: PrivateKey,
    pub public: PublicKey,
    pub address: String,
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
            println!("committing txn: {txn:?}");

            match txn.verify() {
                Ok(verified) => println!("verification: {verified}"),
                Err(e) => println!("Something went wrong verifiying txn: {e}"),
            };

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

        //addy
        let hash = sha256::digest(key.public_key().as_bytes());
        let address = hash.into_bytes().to_base58();

        //println!("Address: {}", address);

        Key {
            private: key,
            public: key.public_key(),
            address: address,
        }
    }
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u128, key: &Key) -> Transaction {
        let mut txn = Transaction {
            amount,
            from,
            to,
            sender_public_key: String::from(""),
            hash: String::from(""),
            signature: String::from(""),
        };

        let serialized_txn =
            serde_json::to_string(&(&txn.from, &txn.to, amount)).expect("nice try bruh");
        let tx_hash = sha256::digest(&serialized_txn);
        txn.sender_public_key = hex::encode(key.public.as_bytes());
        txn.signature = hex::encode(key.private.sign(&tx_hash).as_bytes());
        txn.hash = tx_hash;
        txn
    }
    pub fn spend(self, blockchain: &mut Blockchain) {
        //let verificaiton = key.public.verify(signature, tx_hash);
        //println!("verification: {verificaiton:?}");
        //genesis block can spend whatev
        if blockchain.blocks.len() != 0 {
            if let Some(from) = blockchain.accounts.get(&self.from) {
                if from < &self.amount {
                    return;
                }
            }
        }
        blockchain.pending_transactions.push(self);
    }

    pub fn verify(&self) -> Result<bool, String> {
        let public_key_bytes = hex::decode(&self.sender_public_key).expect("Error decoding");
        let public_key = PublicKey::from_bytes(&public_key_bytes).expect("Error public key");

        let signature_bytes = hex::decode(&self.signature).expect("error sig bytes");
        let signature = Signature::from_bytes(&signature_bytes).expect("error w sig");

        let result = public_key.verify(signature, &self.hash);

        println!("result: {result}!!!!!!!!!!!!!!!!!");

        Ok(result)
    }
}
