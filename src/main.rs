use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::CliArgs::parse();
    // println!("{:?}", args.command);

    match args.command {
        args::Command::Encode(args) => commands::encode(args)?,
        args::Command::Decode(args) => commands::decode(args)?,
        args::Command::Remove(args) => commands::remove(args)?,
        args::Command::Print(args) => commands::print(args)?,
    }
    // println!("Sucess!");
    Ok(())
}