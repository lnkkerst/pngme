#![warn(clippy::all)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Commands;
use clap::Parser;

use crate::args::Cli;
pub use crate::chunk::Chunk;
pub use crate::chunk_type::ChunkType;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Encode(args) => {
            commands::encode(args)?;
        }
        Commands::Decode(args) => {
            if let Some(message) = commands::decode(args) {
                println!(
                    "The message in chunk type `{}` is `{}`",
                    args.chunk_type, message
                );
            } else {
                println!("No message found for chunk type `{}`", args.chunk_type);
            }
        }
        Commands::Remove(args) => {
            commands::remove(args)?;
        }
        Commands::Print(args) => {
            commands::print_chunks(args)?;
        }
    }
    Ok(())
}
