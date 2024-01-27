use blake2::digest::Key;
use bls_signatures::{self, Serialize};
use currychain::chain_utils::Transaction;
use serde;
use std::io::BufRead;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use currychain::chain_utils;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader, WriteHalf},
    net::TcpListener,
    sync::broadcast::{self, Receiver, Sender},
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Client connected to server");

    // Send a message to the server
    let message = b"Hello from client";
    println!("Message sent to server");

    let mut chain = chain_utils::Blockchain::new();

    let key = chain_utils::Key::new();
    let txn = Transaction::new(key.address.clone(), key.address.clone(), 100, &key);

    let serialized = serde_json::to_string(&txn).expect("bad");

    stream.write_all(serialized.as_bytes()).await.unwrap();
    //txn.spend(&mut chain);
    //chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);
}
