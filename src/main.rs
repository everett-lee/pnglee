mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;

use args::Args;
use clap::Parser;
use commands::Command;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Encode => return Command::handle_encode(args),
        Command::Decode => return Command::handle_decode(args),
        Command::Remove => return Command::handle_remove(args),
        Command::Print => return Command::handle_print(args),
    }
}
