use crate::chunk_type::ChunkType;

use anyhow::Result;
use anyhow::anyhow;

struct Chunk {
    chunk_type: ChunkType,
    data: [u8; 4],
    crc: u32
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;


    fn try_from(bytes: &[u8]) -> Result<Self> {
        // if !ChunkType::is_bytes_valid(bytes) {
        //     return Err(anyhow!("Provided byte array not valid"));
        // }
        // TODO check total length of btyes if >= 8

        let length_bytes = &bytes[..4];
        let length = u32::from_be_bytes(length_bytes.try_into().unwrap());

        let chunk_type_bytes = &bytes[4..8];
        let chunk_type = ChunkType::try_from(chunk_type_bytes.into());

        let data_bytes = &bytes[8..];


        Ok(Chunk{b: bytes})
    }
}