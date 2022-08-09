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
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Args, Debug)]
pub struct Encode {
    #[clap(value_parser)]
    file_path: PathBuf,
    #[clap(value_parser)]
    chunk_type: String,
    #[clap(value_parser)]
    message: String,
    #[clap(value_parser)]
    output_file: Option<PathBuf>
}

#[derive(Args, Debug)]
pub struct Decode {
    #[clap(value_parser)]
    file_path: PathBuf,
    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Remove {
    #[clap(value_parser)]
    file_path: PathBuf,
    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct Print {
    #[clap(value_parser)]
    file_path: PathBuf,
}