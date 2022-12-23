use std::{path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};

use crate::{chunk_type::ChunkTypeError, ChunkType};

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
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,

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
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// PNG file to remove message
    pub file_path: PathBuf,

    /// Chunk type of message to remove
    #[clap(value_parser = chunk_parser)]
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// PNG file
    pub file_path: PathBuf,
}

fn chunk_parser(s: &str) -> Result<ChunkType, String> {
    match ChunkType::from_str(s) {
        Ok(chunk) => Ok(chunk),
        Err(ChunkTypeError::InvalidLength) => Err("Chunk type length must be 4".to_string()),
        Err(ChunkTypeError::InvalidByte) => {
            Err("Chunk type must consist of uppercase and lowercase ASCII letters".to_string())
        }
    }
}
