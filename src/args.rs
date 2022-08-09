use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Encode a messages into a file in chunk of given type, overwriting existing chunks of that type
    Encode(Encode),
    /// Decode a message from a file from given chunk type, printing to stdout
    Decode(Decode),
    /// Remove all chunks of the type (and their data) from the file
    Remove(Remove),
    /// Print all chunk data that can be parsed as character strings
    Print(Print),
}

#[derive(Args, Debug)]
pub struct Encode {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
    #[clap(value_parser)]
    pub output_file: Option<PathBuf>
}

#[derive(Args, Debug)]
pub struct Decode {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Remove {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Print {
    #[clap(value_parser)]
    pub file_path: PathBuf,
}