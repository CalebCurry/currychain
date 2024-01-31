mod args;
mod chain_utils;
mod handlers;

use crate::{args::AccountSubcommand, handlers::handle_command};
use args::Sub;
use clap::{parser, Parser};

fn main() {
    let args = Sub::parse();
    println!("args: {args:?}");
    handle_command(args.category);
}
