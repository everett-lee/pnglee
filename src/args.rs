use crate::commands;
use commands::Command;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub command: Command,

    #[arg()]
    pub file_path: Option<String>,

    #[arg()]
    pub chunk_type: Option<String>,

    #[arg()]
    pub message: Option<String>,

    #[arg()]
    pub output_file: Option<String>
}
