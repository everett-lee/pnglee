
use core::fmt;
use std::{any, fmt::{Display, Formatter}, str::FromStr};

use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
pub struct ChunkType {
    b: [u8; 4]
}


impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.b
    }

    fn is_bytes_valid(bytes: [u8; 4]) -> bool {
        bytes.into_iter()
        .all(|b| (b >= 65 && b <= 90) || (b >= 97 && b <= 122))
    }

    fn is_bit_unset(&self, byte_position: usize, bit_position: u8) -> bool {
        // Check byte at byte_position in array to see if bit at bit_position is 0.
        // bit_position is zero-indexed from the right side
        // This is achieved by applying a mask of the bit at bit_position.
        // E.g. 82 in binary is 01010010
        // Applying mask with bit_position = 5, or 00100000
        // Result is 0 
        if byte_position >= 8 {
            panic!("Byte position must be between 0 and 3");
        }

        if bit_position >= 8 {
            panic!("Bit position must be between 0 and 7");
        }
        let byte = self.bytes()[byte_position];
        let mask = 1 << bit_position;
        byte & mask == 0
    }

    fn is_critical(&self) -> bool {
        self.is_bit_unset(0, 5)
    }

    fn is_public(&self) -> bool {
        self.is_bit_unset(1, 5)
    }    
    
    fn is_reserved_bit_valid(&self) -> bool {
        self.is_bit_unset(2, 5)
    }   

    fn is_safe_to_copy(&self) -> bool {
        !self.is_bit_unset(3, 5)
    }
    
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && ChunkType::is_bytes_valid(self.bytes())
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = std::str::from_utf8(&self.b).expect("ChunkType should only contain valid UTF-8 characters");
        write!(f, "{}", s)
    }
}


impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;


    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !ChunkType::is_bytes_valid(bytes) {
            return Err(anyhow!("Provided byte array not valid"));
        }

        Ok(ChunkType{b: bytes})
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes: [u8; 4] = s.as_bytes().try_into().expect("Length of string should be 4");

        if !ChunkType::is_bytes_valid(bytes) {
            return Err(anyhow!("Provided byte array not valid"));
        }

        Ok(ChunkType {b: bytes})
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
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
