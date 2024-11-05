mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;

use args::Args;
use clap::Parser;
use commands::Command;

pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Encode => return Command::handle_encode(args),
        Command::Decode => {
            let _ = Command::handle_decode(args);
            return Ok(());
        }
        Command::Remove => return Command::handle_remove(args),
        Command::Print => {
            let _ = Command::handle_print(args);
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{args::Args, commands::Command};

    #[test]
    fn itest() {
        let input_path = "./capy.png";
        let chunk_type = "teSt";
        let test_msg = "my test message";
        let output_path = "./test-capy.png";

        let encode_args = Args {
            command: Command::Encode,
            file_path: Some(String::from(input_path)),
            chunk_type: Some(String::from(chunk_type)),
            message: Some(String::from(test_msg)),
            output_file: Some(String::from(output_path)),
        };
        Command::handle_encode(encode_args).unwrap();

        let decode_args = Args {
            command: Command::Decode,
            file_path: Some(String::from(output_path)),
            chunk_type: Some(String::from(chunk_type)),
            message: None,
            output_file: None,
        };
        let decode_res = Command::handle_decode(decode_args.clone()).unwrap();
        assert!(decode_res == test_msg);

        let print_args = Args {
            command: Command::Print,
            file_path: Some(String::from(output_path)),
            chunk_type: None,
            message: None,
            output_file: None,
        };
        let print_res = Command::handle_print(print_args).unwrap();
        assert!(print_res == vec![test_msg]);

        let remove_args = Args {
            command: Command::Remove,
            file_path: Some(String::from(output_path)),
            chunk_type: Some(String::from(chunk_type)),
            message: None,
            output_file: None,
        };
        let _ = Command::handle_remove(remove_args).unwrap();

        let decode_res = Command::handle_decode(decode_args).unwrap();
        assert!(decode_res == String::from(""));
    }
}
