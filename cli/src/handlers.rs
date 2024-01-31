use crate::args::{AccountSubcommand, Category, SendArgs};
use crate::chain_utils;
use crate::chain_utils::Key;
pub fn handle_command(category: Category) {
    match category {
        Category::Account(account_command) => match account_command.command {
            AccountSubcommand::New => account_new(),
            AccountSubcommand::List => account_list(),
            AccountSubcommand::Send(args) => account_send(args),
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

pub fn account_send(args: SendArgs) {
    let key = Key::from_str(&args.privateKey);
    let amount = args.amount;
    let to = args.to.clone();
    let txn = chain_utils::Transaction::new(key.address.clone(), args.to, amount, &key);
}

pub fn account_list() {
    // Logic for 'account list'
    println!("Handling account list");
}

pub fn node() {
    // Logic for 'node' command
    println!("Handling node command");
}
