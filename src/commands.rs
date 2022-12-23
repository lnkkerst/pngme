use std::{fs, str::FromStr};

use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    png::Png,
    Chunk, ChunkType,
};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: &EncodeArgs) -> crate::Result<()> {
    let EncodeArgs {
        file_path,
        chunk_type,
        message,
        output,
    } = args;
    let mut png = Png::try_from(&fs::read(file_path)?[..])?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);
    let output = match output {
        Some(path) => path,
        None => file_path,
    };
    fs::write(output, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: &DecodeArgs) -> Option<String> {
    let DecodeArgs {
        file_path,
        chunk_type,
    } = args;
    let png = Png::try_from(&fs::read(file_path).ok()?[..]).ok()?;
    let chunk = png.chunk_by_type(chunk_type)?;
    chunk.data_as_string().ok()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: &RemoveArgs) -> crate::Result<()> {
    let RemoveArgs {
        file_path,
        chunk_type,
    } = args;
    let mut png = Png::try_from(&fs::read(file_path)?[..])?;
    png.remove_chunk(chunk_type)?;
    fs::write(file_path, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: &PrintArgs) -> crate::Result<()> {
    let PrintArgs { file_path } = args;
    let png = Png::try_from(&fs::read(file_path)?[..])?;
    let mut count = 0;
    println!("Detect all chunks wich message(may be incorrect)...");
    for chunk in png.chunks() {
        if let Ok(message) = chunk.data_as_string() {
            if !message.trim().is_empty() {
                count += 1;
                println!(
                    "Found chunk {}, the type is `{}`, the message is `{}`",
                    count,
                    chunk.chunk_type(),
                    message
                );
            }
        }
    }
    println!("{} results in total", count);
    Ok(())
}
