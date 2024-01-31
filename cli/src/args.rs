use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Sub {
    #[clap(subcommand)]
    pub category: Category,
}

#[derive(Debug, Subcommand)]
pub enum Category {
    ///account commands
    Account(AccountCommand),
    Node,
}

#[derive(Debug, Args)]
pub struct AccountCommand {
    #[clap(subcommand)]
    pub command: AccountSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AccountSubcommand {
    ///Generate a new keypair
    New,
    List,
    Send(SendArgs),
}

#[derive(Debug, Args)]
pub struct SendArgs {
    #[clap(short, long)]
    pub amount: u128,
    #[clap(short, long)]
    pub to: String,
    #[clap(short, long)]
    pub privateKey: String,
}
