#![warn(clippy::all)]
#![allow(unused)]

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

fn main() {
    let cli = Cli::parse();
}
