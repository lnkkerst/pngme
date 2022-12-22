#![warn(clippy::all)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub use crate::chunk_type::ChunkType;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    todo!()
}
