#![allow(dead_code)]

use crate::args::{Encode, Decode, Remove, Print};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

use std::str::FromStr;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context, bail};


pub fn encode(args: Encode) -> Result<()> {
    // the png specification says that the series of chunks must be wrapped
    // in IHDR and IEND on the beginning and end, so I think if we just
    // appended a new chunk it wouldn't be a valid PNG anymore
    // we can try it first though
    let mut image = png_from_file(&args.file_path).context("Failed to get Png from file")?;

    // remove existing chunk (just one) - therefore it shouldn't be possible to have more than 1
    // of a custom chunk in a file strictly through use of this program
    let _ = image.remove_chunk(&args.chunk_type);

    let msg_chunk = Chunk::new(ChunkType::from_str(&args.chunk_type)?, args.message.into_bytes());
    // let end_chunk = image.remove_chunk("IEND").context("Failed to find the IEND chunk")?;
    image.append_chunk(msg_chunk);
    // image.append_chunk(end_chunk);

    let output_path = match args.output_file {
        Some(path) => path,
        None => args.file_path,
    };

    fs::write(output_path, image.as_bytes()).context("Failed to write Png to file")?;

    Ok(())
}

pub fn decode(args: Decode) -> Result<()> {
    let image = png_from_file(&args.file_path).context("Failed to get Png from file")?;
    let first_chunk = image.chunk_by_type(&args.chunk_type);
    let data = match first_chunk {
        Some(chunk) => chunk.data_as_string()?,
        None => bail!("No chunk with given type found"),
    };

    println!("{data}");
    Ok(())
}

pub fn remove(args: Remove) -> Result<()> {
    // this is going to be really inefficient since png::remove iterates through
    // the whole vec of chunks every time
    let mut image = png_from_file(&args.file_path).context("Failed to get Png from file")?;

    while let Ok(_chunk) = image.remove_chunk(&args.chunk_type) {
        // do nothing, feels a bit like heresy though
    }

    fs::write(args.file_path, image.as_bytes()).context("Failed to write Png to file")?;

    Ok(())
}

pub fn print(args: Print) -> Result<()> {
    let image = png_from_file(&args.file_path).context("Failed to get Png from file")?;
    for chunk in image.chunks() {
        print!("{}: {}\n", chunk.chunk_type(), chunk.data_as_string().unwrap_or("".to_string()));
    }
    Ok(())
}

fn png_from_file(path: &PathBuf) -> Result<Png> {
    Png::try_from(fs::read(path).context("Failed reading file")?.as_slice()).context("Failure constructing Png from file")
}