use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;

use args::Args;
use chunk::Chunk;
use chunk_type::ChunkType;
use commands::Command;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    fn handle_encode(args: Args) -> Result<()> {
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

    fn handle_decode(args: Args) -> Result<()> {
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

    fn handle_remove(args: Args) -> Result<()> {
        println!("ACTION: {}", args.command); 

        match args.file_path {
            Some(fp) => println!("FILE PATH: {}", fp),
            None => { return Err(anyhow!("No file path provided")) }
        }
        match args.chunk_type {
            Some(ct) => println!("CHUNK TYPE: {}", ct),
            None => { return Err(anyhow!("No chunk type provided")) }
        }
        Ok(())
    }

    fn handle_print(args: Args) -> Result<()> {
        println!("ACTION: {}", args.command); 

        match args.file_path {
            Some(fp) => println!("FILE PATH: {}", fp),
            None => { return Err(anyhow!("No file path provided")) }
        }
        Ok(())
    }


    let args = Args::parse();
    match args.command {
        Command::Encode => return handle_encode(args),
        Command::Decode => return handle_decode(args),
        Command::Remove => return handle_remove(args),
        Command::Print => return handle_print(args),
    }
}
