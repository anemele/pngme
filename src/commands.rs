use std::{fs, path::Path, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode(
    file: impl AsRef<Path>,
    chunk_type: &str,
    message: &str,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let bytes = fs::read(file)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type)?,
        message.bytes().collect(),
    ));
    fs::write(output, png.as_bytes())?;
    Ok(())
}

pub fn decode(file: impl AsRef<Path>, chunk_type: &str) -> anyhow::Result<String> {
    let bytes = fs::read(file)?;
    let png = Png::try_from(bytes.as_slice())?;

    let Some(chunk) = png.chunk_by_type(chunk_type) else {
        anyhow::bail!("not found");
    };

    Ok(chunk.data_as_string()?)
}

pub fn remove(file: impl AsRef<Path>, chunk_type: &str) -> anyhow::Result<String> {
    let bytes = fs::read(&file)?;
    let mut png = Png::try_from(bytes.as_slice())?;

    let chunk = png.remove_first_chunk(chunk_type)?;
    fs::write(file, png.as_bytes())?;

    Ok(chunk.data_as_string()?)
}

pub fn print(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let bytes = fs::read(file)?;
    let png = Png::try_from(bytes.as_slice())?;

    for chunk in png.chunks() {
        let Ok(msg) = chunk.data_as_string() else {
            continue;
        };
        if msg.len() == 0 {
            continue;
        }
        println!();
        println!("type: {}", chunk.chunk_type());
        println!("msg : {}", msg);
    }

    Ok(())
}
