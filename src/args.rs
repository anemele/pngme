use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
pub enum PngMeArgs {
    /// encode message into a PNG file
    /// and save as a new file.
    Encode {
        file: PathBuf,
        chunk_type: String,
        message: String,

        #[arg(short, long, default_value = "output.png")]
        output: Option<PathBuf>,
    },

    /// decode message from a PNG file by chunk-type
    Decode { file: PathBuf, chunk_type: String },

    /// remove chunk from a PNG file by chunk-type
    Remove { file: PathBuf, chunk_type: String },

    /// print ALL message from a PNG file
    Print { file: PathBuf },
}
