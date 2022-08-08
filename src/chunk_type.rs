#![allow(dead_code)]
use std::{str::FromStr, fmt::Display};

use anyhow::{anyhow, Result};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

const FIFTH_BIT_MASK: u8 = 0x20;

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_valid(&self) -> bool {
        // as far as i can tell validity consists of:
        // - reserved bit (3rd byte) must be 0 (uppercase)
        // - safe-to-copy bit (4th byte) must be 0 if ancillary bit (1st byte) is 0 (critical chunk)
        let a = self.is_reserved_bit_valid();
        let _b = self.is_critical();
        let _c = self.is_safe_to_copy();
        // below is what I believe to be the correct definition of validity, but the tests seem to
        // indicate that validity is purely the validity of the reserved bit...
        // a && (if b {!c} else {true})
        a
    }

    pub fn is_critical(&self) -> bool {
        (self.chunk_type[0] & FIFTH_BIT_MASK) == 0
    }

    pub fn is_public(&self) -> bool {
        (self.chunk_type[1] & FIFTH_BIT_MASK) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.chunk_type[2] & FIFTH_BIT_MASK) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.chunk_type[3] & FIFTH_BIT_MASK) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if value.iter().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_lowercase()) {
            Ok(Self {chunk_type: value})
        } else {
            Err(anyhow!("got non-character values for chunk type"))
        }
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 4 {
            let buf: [u8; 4] = <[u8; 4]>::try_from(s.as_bytes())?;
            ChunkType::try_from(buf)
        } else {
            Err(anyhow!("got non 4-byte str for chunk type"))
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {write!(f, "{}", std::str::from_utf8_unchecked(&self.chunk_type[..])) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
