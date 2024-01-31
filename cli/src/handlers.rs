use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::args::{AccountSubcommand, Category, SendArgs};
use crate::chain_utils::Key;
use crate::chain_utils::{self, Transaction};
pub async fn handle_command(category: Category) {
    match category {
        Category::Account(account_command) => match account_command.command {
            AccountSubcommand::New => account_new(),
            AccountSubcommand::List => account_list(),
            AccountSubcommand::Send(args) => account_send(args).await,
        },
        Category::Node => {
            node();
        }
    }
}

pub fn account_new() {
    let key = Key::new();

    println!("\nPrivate key: {:?}\n", key.private);
    println!("Address: {:?}\n", key.address);
}

pub async fn account_send(args: SendArgs) {
    let key = Key::from_str(&args.privateKey);
    let amount = args.amount;
    let to = args.to.clone();
    let txn = chain_utils::Transaction::new(key.address.clone(), args.to, amount, &key);
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Client connected to server");

    // Send a message to the server
    let message = b"Hello from client";
    println!("Message sent to server");

    let mut chain = chain_utils::Blockchain::new();

    let key = chain_utils::Key::new();

    let serialized = serde_json::to_string(&txn).expect("bad");

    stream.write_all(serialized.as_bytes()).await.unwrap();
    //txn.spend(&mut chain);
    //chain.create_block();

    println!("\n\naccounts: {:?}\n\n", chain.accounts);
}

pub fn account_list() {
    // Logic for 'account list'
    println!("Handling account list");
}

pub fn node() {
    // Logic for 'node' command
    println!("Handling node command");
}
