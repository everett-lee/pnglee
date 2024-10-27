use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Bytes;
use crate::chunk_type::ChunkType;

use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let combined: Vec<u8> = chunk_type.bytes().iter().chain(data.iter()).copied().collect();
        let checksum = Chunk::CRC.checksum(&combined);
        Chunk {length, chunk_type, data, crc: checksum}
    }

    pub fn crc(&self) -> u32 {
        let type_bytes = self.chunk_type.bytes();
        let combined: Vec<u8> = type_bytes.iter().chain(self.data.iter()).copied().collect();
        Chunk::CRC.checksum(&combined)
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data_as_string(&self) -> Result<String> {
        if self.data.len() == 0 {
            return Err(anyhow!("No data to convert to String"));
        }
        let res = String::from_utf8(self.data.clone());
        Ok(res?)
    }

    pub fn chunk_data_length(bytes: &[u8]) -> u32 {
        let length_bytes = &bytes[..4];
        u32::from_be_bytes(length_bytes.try_into().unwrap())
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;


    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            return Err(anyhow!("PNG spec requires at least 12 bytes"));
        }

        let length = Chunk::chunk_data_length(bytes);

        let chunk_type_bytes = &bytes[4..8];
        let chunk_type_arr: [u8; 4] = chunk_type_bytes.try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_arr)?;

        if !chunk_type.is_valid() {
            return Err(anyhow!("Chunk type not"));
        }

        let data: Vec<u8> = bytes[8..bytes.len() - 4].into();
        if data.len() != length as usize {
            return Err(anyhow!("Data length does not match specified length"));
        }

        let checksum = Chunk::CRC.checksum(&bytes[4..bytes.len() - 4]);
        let last_four_bytes: [u8; 4] = bytes[bytes.len() - 4..].try_into().unwrap();
        let check_sum_value = u32::from_be_bytes(last_four_bytes);
        
        if checksum != check_sum_value {
            return Err(anyhow!("Checksum invalid"));
        }

        Ok(Chunk{
            length,
            chunk_type,
            data,
            crc: checksum
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
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