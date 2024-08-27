use std::fmt::Display;

use crate::chunk_type::ChunkType;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

fn get_crc32(data: &[u8]) -> u32 {
    crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&data)
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let tmp = value[..4].try_into()?;
        let length = u32::from_be_bytes(tmp);
        let tmp: [u8; 4] = value[4..8].try_into()?;
        let chunk_type = ChunkType::try_from(tmp)?;
        let end = (length + 8) as usize;
        let data = &value[8..end];
        let tmp = value[end..end + 4].try_into()?;
        let crc_read = u32::from_be_bytes(tmp);
        let crc_computed = get_crc32(&value[4..end]);
        if crc_read == crc_computed {
            Ok(Self {
                length,
                chunk_type,
                chunk_data: data.to_vec(),
                crc: crc_read,
            })
        } else {
            Err(Self::Error::from("error crc"))
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunk_type)
    }
}

#[allow(dead_code)]
impl Chunk {
    pub const BYTE_LEN_LENGTH: usize = 4;
    pub const BYTE_LEN_TYPE: usize = 4;
    pub const BYTE_LEN_CRC: usize = 4;
    pub const BYTE_LEN_META: usize =
        Self::BYTE_LEN_LENGTH + Self::BYTE_LEN_TYPE + Self::BYTE_LEN_CRC;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let tmp: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc32 = get_crc32(&tmp);
        Self {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc: crc32,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> crate::Result<String> {
        let s = std::str::from_utf8(&self.chunk_data)?;
        Ok(s.to_string())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
