use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::PngMeArgs;
use commands::{decode, encode, print, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();
    match args {
        PngMeArgs::Encode {
            file,
            chunk_type,
            message,
            output,
        } => encode(file, &chunk_type, &message, output.unwrap_or_default())?,
        PngMeArgs::Decode { file, chunk_type } => {
            let msg = decode(file, &chunk_type)?;
            println!("{}", msg)
        }
        PngMeArgs::Remove { file, chunk_type } => {
            let msg = remove(file, &chunk_type)?;
            println!("{}", msg)
        }
        PngMeArgs::Print { file } => print(file)?,
    }

    Ok(())
}
