

use core::fmt;
use std::str::FromStr;

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