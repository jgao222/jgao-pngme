use std::fmt::Display;
use std::string::String;
use crc::{Crc, CRC_32_ISO_HDLC};
use anyhow::{Result, Error, anyhow, bail};
use crate::chunk_type::ChunkType;


// The algorithm listed on the PNG spec page says the CRC algorithm used is
// specified by ISO-3099. The polynomial given seems to match the one in
// ISO-HDLC so that is what we use
const CRC_CALC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub struct Chunk {
    len: u32,
    c_type: ChunkType,
    crc: u32,
    data: Vec<u8>,
}

impl Chunk {
    #![allow(dead_code)]
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut digest = CRC_CALC.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&data[..]);
        Chunk {len: data.len() as u32, c_type: chunk_type, crc: digest.finalize(), data}
    }

    /// returns the length of the DATA in this chunk in number of bytes
    pub fn length(&self) -> u32 {
        self.len
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.c_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        println!("{}", String::from_utf8_lossy(self.data.as_ref()));
        if let Ok(s) = String::from_utf8(self.data.clone()) {
            Ok(s)
        } else {
            Err(anyhow!("failed to parse data as a string"))
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes_out = Vec::with_capacity((4 + 4 + 4 + self.len) as usize);
        bytes_out.extend_from_slice(&self.len.to_be_bytes());
        bytes_out.extend_from_slice(&self.c_type.bytes());
        bytes_out.extend(&self.data);
        bytes_out.extend(self.crc.to_be_bytes());
        bytes_out
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {  // 4 bytes for length, type, and crc
            bail!("not enough data to form one chunk")
        }
        let size = u32::from_be_bytes(value[0..4].try_into()?);
        let start_of_crc = 8 + size as usize;
        if start_of_crc > value.len() - 3 {  // if laste byte is n, then start_of_crc can be at most at n-3
            bail!("data length mismatch, not enough bytes for CRC")
        }
        let crc = u32::from_be_bytes(value[start_of_crc..start_of_crc + 4].try_into()?);
        let mut data = Vec::with_capacity(size.try_into()?);
        data.extend_from_slice(&value[8..start_of_crc]);

        let ctype_slice: [u8; 4] = value[4..8].try_into()?;
        let c_type = ChunkType::try_from(ctype_slice)?;

        let chunk = Chunk::new(c_type, data);

        let computed_crc = chunk.crc();
        if crc != computed_crc {
            bail!("computed CRC mismatch")
        }

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:x?}, {})", self.len, self.c_type, self.data, self.crc)
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
