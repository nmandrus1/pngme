use crate::chunk::Chunk;

use super::args::Args;
use super::chunk_type::{ChunkType, FromStr};
use super::png::Png;

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

pub fn run_cmd(cli: Args) -> anyhow::Result<()> {
    match cli {
        Args::Decode {file, ctype, all} => decode(file, ctype, all)?,
        Args::Encode {file, ctype, message, output } => encode(file, ctype, message, output)?,
        Args::Remove {file, ctype, all} => remove(file, ctype, all)?,
        Args::Print  {file} => print(file)?,
    }

    Ok(())
}

fn decode(file: PathBuf, ctype: String, all: bool) -> anyhow::Result<()> {
    use std::io::{stdout, BufWriter, Write};
    let mut buf = BufWriter::new(stdout());

    // Read in bytes from file
    let bytes = fs::read(file)?;

    // Create png from bytes
    let png = Png::try_from(&bytes[..])?;

    if all {
        match png.chunks_by_type(&ctype) {
            Some(iter) => iter.for_each(|c| {
                match c.data_as_string() {
                    Ok(s) => writeln!(buf, "Decoded Message: \"{}\"", s).unwrap(),
                    Err(_) => eprintln!("Data is not valid UTF-8"),
                }
            }),
            None => eprintln!("Chunk Type not found"),
        }
    } else {
        match png.chunk_by_type(&ctype) {
            Some(c) => {
                match c.data_as_string() {
                    Ok(s) => println!("Decoded message: \"{}\"", s),
                    Err(_) => eprintln!("Data is not valid UTF-8"),
                }
            }
            None => eprintln!("Chunk Type not found"),
        }; 
    }

    Ok(())
}


fn encode(file: PathBuf, ctype: String, msg: String, output: Option<PathBuf>) -> anyhow::Result<()> {
    // Read in bytes from file
    let bytes = fs::read(&file)?;

    // Create png from bytes
    let mut png = Png::try_from(&bytes[..])?;

    // Get chunk type
    let ctype = ChunkType::from_str(&ctype)?;

    let chunk = Chunk::new(ctype, msg.into_bytes());

    png.append_chunk(chunk);

    match output {
        Some(f) => fs::write(f, png.as_bytes())?,
        None => fs::write(file, png.as_bytes())?
    }

    Ok(())
}


fn remove(file: PathBuf, ctype: String, all: bool) -> anyhow::Result<()> {
    // Read in bytes from file
    let bytes = fs::read(&file)?;

    // Create png from bytes
    let mut png = Png::try_from(&bytes[..])?;

    // Remove chunk with ctype
    if all {
        while png.remove_chunk(&ctype).is_ok() {
            png.remove_chunk(&ctype)?;
        }
        println!("Chunks removed");
    } else {
        png.remove_chunk(&ctype)?;
        println!("Chunk removed");
    }
    
    // Write new png bytes to file
    fs::write(&file, png.as_bytes())?;

    Ok(())
}

// Print all chunk data that contains a valid UTF-8 string
fn print(file: PathBuf) -> anyhow::Result<()> {
    use std::io::{stdout, BufWriter, Write};
    let mut buf = BufWriter::new(stdout());
    
    // Read from bytes
    let bytes = fs::read(&file)?;

    let png = Png::try_from(&bytes[..])?;

    for chunk in png.chunks() {
        if chunk.data_as_string().is_ok() {
            writeln!(buf, "Possible Message: \"{}\"", chunk.data_as_string().unwrap())?
        }
    }

    Ok(())
}
