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
    let txn = Transaction::new(from, "Claire".to_string(), 100, &key);

    //test a fake txn:
    //let key2 = Key::new();
    //let txn = Transaction::new(from, "Claire".to_string(), 100, &key2);

    txn.spend(&mut chain);

    // println!("{:?}", chain.pending_transactions);

    chain.create_block();
    // println!("{:?}", chain.blocks);

    //let from_key2 = hex::encode(key.public.as_bytes());
}
