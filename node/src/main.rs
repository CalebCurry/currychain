use std::env;
use std::io::BufRead;

use bls_signatures::{self, Serialize};
use serde;
pub mod chain_utils;
use chain_utils::{Key, Transaction};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self},
};

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    let mut port;
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        port = args[1].clone();
    } else {
        port = String::from("8080");
    }
    let port_clone = port.clone();
    let server_handle = tokio::spawn(async move {
        let address = format!("127.0.0.1:{port}");
        println!("address: {address}");
        let mut listener = TcpListener::bind(&address).await.unwrap();

        println!("Server listening on {address}");

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

                                //chain.create_block();

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
    let client_handle = tokio::spawn(async move {
        let url = format!("127.0.0.1:{port_clone}");
        println!("client connecting on {}", &url.clone());
        let mut stream = TcpStream::connect(url.clone()).await.unwrap();
        println!("Client connected to server");

        // Example of sending some data
        //stream.write_all(b"Hello from client").await.unwrap();
    });

    let cli_handle = tokio::spawn(async move {
        let mut lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();
        while let data = lines.next_line().await.unwrap().unwrap() {
            println!("result: {:?}", data);
            if data.starts_with("connect ") {
                let addr = data[8..].trim(); // Extract the address
                if let Err(e) = connect_to_peer(addr).await {
                    eprintln!("Failed to connect to peer {}: {}", addr, e);
                }
            }
        }

        println!("done");
    });

    let _ = tokio::try_join!(server_handle, client_handle, cli_handle);

    // let key = Key::new();
    // println!("{:?} public key: {:?}", key.private, key.public);
    // println!("Your address is {}", key.address);
    // //println!("Your public bytes are {:?}", key.public.as_bytes());
    // //println!("Your address is {} characters long", key.address.len());

    // let mut chain = chain_utils::Blockchain::new();

    // let key2 = Key::new();
    // let txn = Transaction::new(key.address.clone(), key2.address.clone(), 100, &key);
    // txn.spend(&mut chain);
    // chain.create_block();

    // println!("\n\naccounts: {:?}\n\n", chain.accounts);

    // let txn2 = Transaction::new(key2.address.clone(), key.address.clone(), 50, &key2);
    // txn2.spend(&mut chain);
    // chain.create_block();

    // println!("\n\naccounts: {:?}\n\n", chain.accounts);
    // println!("{:?}", chain.pending_transactions);

    // println!("{:?}", chain.blocks);

    //let from_key2 = hex::encode(key.public.as_bytes());
}

async fn connect_to_peer(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting to connect to peer at {}", address);
    let mut stream = TcpStream::connect(address).await?;
    println!("Connected to peer at {}", address);

    // Add logic for interacting with the peer...
    // e.g., send/receive messages

    Ok(())
}
