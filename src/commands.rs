
use crate::args::Args;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

use core::fmt;
use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Command {
    Encode,
    Decode,
    Remove, 
    Print
}


impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Encode => write!(f, "Encode"),
            Command::Decode => write!(f, "Decode"),
            Command::Remove => write!(f, "Remove"),
            Command::Print => write!(f, "Print"),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "encode" => Ok(Command::Encode),
            "decode" => Ok(Command::Decode),
            "remove" => Ok(Command::Remove),
            "print" => Ok(Command::Print),
            _ => Err(format!("Invalid action: '{}'. Use one of: [encode, decode, remove, print].", s)),
        }
    }
}

impl Command {
    pub fn handle_encode(args: Args) -> Result<()> {
        let file_path = args.file_path
          .ok_or_else(|| anyhow::anyhow!("No file path provided"))?;
        let chunk_type = args.chunk_type
        .ok_or_else(|| anyhow::anyhow!("No chunk type provided"))?;
        let message = args.message
        .ok_or_else(|| anyhow::anyhow!("No message provided"))?;

        println!("Encoding file with message {}", &message); 
        let contents = fs::read(&file_path)?;
        let parsed_chunk_type = ChunkType::from_str(&chunk_type)?;
        if !parsed_chunk_type.is_valid() {
            return Err(anyhow!("Provided chunk type not valid"))
        }

        let chunk = Chunk::new(parsed_chunk_type, message.as_bytes().to_vec());
        let mut png = Png::try_from(contents.as_ref())?;
        png.append_chunk(chunk);

        match &args.output_file {
            Some(op) => {
                fs::write(op, png.as_bytes())?;
            },
            None => {
                fs::write(&file_path, png.as_bytes())?;
            }
        }        
        Ok(())
    }

    pub fn handle_decode(args: Args) -> Result<()> {
        let file_path = args.file_path
          .ok_or_else(|| anyhow::anyhow!("No file path provided"))?;
        let chunk_type = args.chunk_type
        .ok_or_else(|| anyhow::anyhow!("No chunk type provided"))?;

        let contents = fs::read(&file_path)?;
        let png = Png::try_from(contents.as_ref())?;
        let secret_chunk = png.chunk_by_type(&chunk_type);

        match secret_chunk {
            Some(c) => {
                let secret_message = c.data_as_string()?;
                println!("Super secret message: {}", secret_message);
            }
            None => {
                println!("No chunk found for type {}", &chunk_type);
            }
        }
        Ok(())
    }

    pub fn handle_remove(args: Args) -> Result<()> {
        let file_path = args.file_path
        .ok_or_else(|| anyhow::anyhow!("No file path provided"))?;
      let chunk_type = args.chunk_type
      .ok_or_else(|| anyhow::anyhow!("No chunk type provided"))?;
        println!("Removing for chunk type {}", chunk_type); 

        let contents = fs::read(&file_path)?;
        let mut png = Png::try_from(contents.as_ref())?;
        png.remove_first_chunk(&chunk_type)?;
        fs::write(&file_path, png.as_bytes())?;
        Ok(())
    }

    pub fn handle_print(args: Args) -> Result<()> {
        let file_path = args.file_path
        .ok_or_else(|| anyhow::anyhow!("No file path provided"))?;

        let contents = fs::read(&file_path)?;
        let png = Png::try_from(contents.as_ref())?;

        for chunk in png.chunks() {
            if !chunk.chunk_type().is_critical() && !chunk.chunk_type().is_public() {
                println!("{}", chunk);
            }
        }
        Ok(())
    }

}