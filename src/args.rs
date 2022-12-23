use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a message into a PNG file
    Encode(EncodeArgs),

    /// Decode a message stored in a PNG file
    Decode(DecodeArgs),

    /// Remove a message from a PNG file
    Remove(RemoveArgs),

    /// Print a list of PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// PNG file to store message
    pub file_path: PathBuf,

    /// Chunk type for storing message
    pub chunk_type: String,

    /// Message to be stored
    pub message: String,

    /// Write the output PNG file to specific location
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// PNG file to decode message
    pub file_path: PathBuf,

    /// Chunk type of message to decode
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// PNG file to remove message
    pub file_path: PathBuf,

    /// Chunk type of message to remove
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// PNG file
    pub file_path: PathBuf,
}
