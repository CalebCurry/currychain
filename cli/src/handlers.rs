use crate::args::{AccountSubcommand, Category};
use crate::chain_utils::Key;

pub fn handle_command(category: Category) {
    match category {
        Category::Account(account_command) => match account_command.command {
            AccountSubcommand::New => account_new(),
            AccountSubcommand::List => account_list(),
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

pub fn account_list() {
    // Logic for 'account list'
    println!("Handling account list");
}

pub fn node() {
    // Logic for 'node' command
    println!("Handling node command");
}
