use base58::ToBase58;
use bls_signatures::{PrivateKey, PublicKey, Serialize, Signature};
use hex;
use num_bigint::{BigInt, Sign};
use rand::thread_rng;
use ripemd::{Digest, Ripemd160};
use serde;
use sha256;
use std::{collections::HashMap, num::ParseIntError};

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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

fn calculate_checksum(addy: &str) -> String {
    //calculate a checksum
    //double hash just cuz
    //get first 4 bytes
    //append to addy
    let addy_hash = sha256::digest(addy);
    let addy_hash_hash = sha256::digest(addy_hash);
    let checksum = &addy_hash_hash[0..4];
    checksum.to_string()
}

fn calculate_addy_with_checksum(addy_no_checksum: String) -> String {
    let checksum = calculate_checksum(&addy_no_checksum);
    format!("{addy_no_checksum}{checksum}")
}

// pub fn hex_to_decimal(hex: &str) -> Result<String, ParseIntError> {
//     // Remove the '0x' prefix if present
//     let hex = if hex.starts_with("0x") {
//         &hex[2..]
//     } else {
//         hex
//     };

//     // Convert hex to BigInt
//     match BigInt::from_str_radix(hex, 16) {
//         Ok(decimal) => Ok(decimal.to_string()),
//         Err(e) => Err(e),
//     }
// }

impl Key {
    pub fn from_str(source: &str) -> Key {
        let hex_no_prefix = source.trim_start_matches("0x");
        let private = bls_signatures::PrivateKey::from_bytes(
            hex::decode(hex_no_prefix).expect("decode error").as_slice(),
        );
        Self::generate_address(private.expect("bad input"))
    }

    pub fn new() -> Key {
        let private = bls_signatures::PrivateKey::generate(&mut thread_rng());
        Self::generate_address(private)
    }
    fn generate_address(key: bls_signatures::PrivateKey) -> Key {
        //addy
        //sha256 -> ripemd160 -> base58 -> prepend -> checksum tbd?
        let hash = sha256::digest(key.public_key().as_bytes());

        let mut hasher = Ripemd160::new();
        hasher.update(hash);
        let hash2 = hasher.finalize();

        let hash2_58 = hash2.to_base58();
        let addy_no_checksum = format!("curry01{hash2_58}");

        //concatenate the checksum on the address
        let addy = calculate_addy_with_checksum(addy_no_checksum);
        println!("Address: {addy}");

        //test to verify this works:
        //let r_checksum = &addy[addy.len() - 4..];

        //let r_addy_no_checksum = &addy[..addy.len() - 4];
        //let r_addy_hash = sha256::digest(r_addy_no_checksum);
        //let r_addy_hash_hash = sha256::digest(&r_addy_hash);

        //let r_calculated_checksum = &r_addy_hash_hash[0..4];

        // println!(
        //     "~CHECKSUM CHECKS OUT: {}",
        //     r_calculated_checksum == r_checksum
        // );

        Key {
            private: key,
            public: key.public_key(),
            address: addy,
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
        println!("\n\nCrafting transaction: {:?}\n\n", txn);
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

#[cfg(test)]
mod tests {
    //invoke tests with cargo test
    //cargo test --verbose -- --nocapture
    //allows you to see console logs
    use super::*;

    #[test]
    fn test_checksum() {
        // Generate a key and an address
        let key = Key::new();
        let address = key.address;

        // Assume the last 4 characters are the checksum
        let checksum_part = &address[address.len() - 4..];

        // Recalculate the checksum
        let address_without_checksum = &address[..address.len() - 4];
        let recalculated_checksum = calculate_checksum(address_without_checksum);

        // Check if the original checksum matches the recalculated checksum
        assert_eq!(
            checksum_part, recalculated_checksum,
            "Checksum does not match!"
        );
    }
    #[test]
    fn test_altered_address_checksum() {
        // Generate a key and an address
        let key = Key::new();
        let address = key.address;

        // Recalculate the checksum
        let addy_no_checksum_unchanged = &address[..address.len() - 4].to_string();

        // Assume the last 4 characters are the checksum
        let checksum = calculate_checksum(addy_no_checksum_unchanged);

        let mut chars: Vec<char> = addy_no_checksum_unchanged.chars().collect();
        chars[5] = 'c';
        let addy_no_checksum: String = chars.into_iter().collect();

        let checksum2 = calculate_checksum(&addy_no_checksum);

        println!("Correct hash: {checksum}, calculated hash from modified addy: {checksum2}");

        assert_ne!(checksum, checksum2, "Checksums with different addys match!");
    }
}
