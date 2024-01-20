use bls_signatures::{self, Serialize};
use chain_utils::{Key, Transaction};
mod chain_utils;
fn main() {
    let key = Key::new();
    println!("{:?} public key: {:?}", key.private, key.public);
    println!("Your address is {}", key.address);
    //println!("Your public bytes are {:?}", key.public.as_bytes());
    //println!("Your address is {} characters long", key.address.len());

    let mut chain = chain_utils::Blockchain::new();

    //hex representation of public key
    let from = hex::encode(key.public.as_bytes());
    let hash = sha256::digest(from.as_bytes());

    let key2 = Key::new();
    let addy = sha256::digest(hex::encode(key2.public.as_bytes()).as_bytes());
    let txn = Transaction::new(hash.clone(), addy.clone(), 100, &key);
    txn.spend(&mut chain);
    chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);

    let txn2 = Transaction::new(addy, hash, 50, &key2);
    txn2.spend(&mut chain);
    chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);
    // println!("{:?}", chain.pending_transactions);

    // println!("{:?}", chain.blocks);

    //let from_key2 = hex::encode(key.public.as_bytes());
}
