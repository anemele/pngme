use clap::Parser;

use std::path::PathBuf;

#[derive(Parser)]
pub enum PngMeArgs {
    /// encode message into a PNG file
    /// and save as a new file.
    Encode {
        /// the PNG file
        file: PathBuf,
        /// four alphabets
        chunk_type: String,
        /// what you want to encode into the PNG file
        message: String,

        #[arg(short, long, default_value = "output.png")]
        output: PathBuf,
    },

    /// decode message from a PNG file by chunk-type
    Decode {
        /// the PNG file
        file: PathBuf,
        /// four alphabets
        chunk_type: String,
    },

    /// remove chunk from a PNG file by chunk-type
    Remove {
        /// the PNG file
        file: PathBuf,
        /// four alphabets
        chunk_type: String,
    },

    /// print ALL message from a PNG file
    Print {
        /// the PNG file
        file: PathBuf,
    },
}
