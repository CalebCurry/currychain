use bls_signatures;
use chain_utils::{Key, Transaction};
use rand::{thread_rng, CryptoRng, RngCore};
mod chain_utils;
fn main() {
    let mut chain = chain_utils::Blockchain::new();
    let txn = Transaction::new("Caleb".to_string(), "Claire".to_string(), 100);
    txn.spend(&mut chain);

    println!("{:?}", chain.pending_transactions);
    println!("{:?}", chain.blocks);

    chain.create_block();

    println!("{:?}", chain.pending_transactions);
    println!("{:?}", chain.blocks);

    // //should fail
    // let txn = Transaction::new("Caleb".to_string(), "Claire".to_string(), 100);
    // txn.spend(&mut chain);
    // chain.create_block();

    let txn = Transaction::new("Claire".to_string(), "Caleb".to_string(), 50);
    txn.spend(&mut chain);
    chain.create_block();
    println!("{:?}", chain.pending_transactions);
    println!("{:?}", chain.blocks);
    println!("{:?}", chain.accounts);

    let key = Key::new();
    println!("{:?} {:?}", key.private, key.public);
}
