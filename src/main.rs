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

    let key2 = Key::new();
    let txn = Transaction::new(key.address.clone(), key2.address.clone(), 100, &key);
    txn.spend(&mut chain);
    chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);

    let txn2 = Transaction::new(key2.address.clone(), key.address.clone(), 50, &key2);
    txn2.spend(&mut chain);
    chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);
    // println!("{:?}", chain.pending_transactions);

    // println!("{:?}", chain.blocks);

    //let from_key2 = hex::encode(key.public.as_bytes());
}
