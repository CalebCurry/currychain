use std::io::BufRead;

use bls_signatures::{self, Serialize};
use serde;
pub mod chain_utils;
use chain_utils::{Key, Transaction};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, WriteHalf},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

async fn handle_transaction(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    // Read data into the buffer
    match socket.read(&mut buf).await {
        Ok(size) => {
            println!("Ok section");
            // Process the transaction data
            // For example, deserialize the buffer into a transaction object
            // and then validate and add it to a pool or blockchain
        }
        Err(e) => {
            println!("Failed to read from socket; err = {:?}", e);
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let server_handle = tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("Server listening on 127.0.0.1:8080");

        let (tx, _) = broadcast::channel(10);

        loop {
            let (mut socket, addr) = listener.accept().await.unwrap();
            let tx = tx.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();
                let mut reader = BufReader::new(reader);
                let mut line = String::new();

                loop {
                    tokio::select! {
                        result = reader.read_line(&mut line) => {
                            if result.unwrap() == 0 {
                                break;
                            }

                            println!("Received {line} from {addr}");
                            tx.send((line.clone(), addr));
                            line.clear();
                        }
                        result = rx.recv() => {
                            let (msg, other_addr) = result.unwrap();

                            let deserialized: Result<Transaction, serde_json::Error> = serde_json::from_str(&msg);
                            match deserialized {
                                Ok(txn) => {
                                    let mut chain = chain_utils::Blockchain::new();
                                    txn.spend(&mut chain);
                                    chain.create_block();

                                    println!("\n\naccounts: {:?}\n\n", chain.accounts);
                                    if addr != other_addr {
                                        writer.write_all(msg.as_bytes()).await.unwrap();
                                    }

                                }
                                err => println!("Invalid txn format")
                            }
                        }


                            //println!("\n\naccounts: {:?}\n\n", chain.accounts);




                    }
                }
            });
        }
    });

    // Start the client component
    let client_handle = tokio::spawn(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        println!("Client connected to server");

        // Example of sending some data
        stream.write_all(b"Hello from client").await.unwrap();
    });

    // Wait for both server and client to finish their tasks
    let _ = tokio::try_join!(server_handle, client_handle);

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
