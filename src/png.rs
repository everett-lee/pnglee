use core::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::chunk;
use crate::chunk::Chunk;

use anyhow::Result;
use anyhow::anyhow;

struct Png {
    chunks: Vec<Chunk>
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn extract_chunk_from_bytes(start: usize, bytes: &[u8]) -> (Chunk, usize) {
        // Length of data plus the 12 extra bytes
        let n_extra_bytes = 12;
        let total_length = Chunk::chunk_data_length(&bytes[start..]) + n_extra_bytes;
        let end = start as usize + total_length as usize;

        let data_bytes = &bytes[start..end];
        (Chunk::try_from(data_bytes).unwrap(), end)
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let joined: String = self.chunks.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", joined)
    }
}


impl TryFrom<&[u8]> for Png {
    type Error = anyhow::Error;


    fn try_from(bytes: &[u8]) -> Result<Self> {
        let header_bytes = &bytes[0..8];
        if header_bytes != &Png::STANDARD_HEADER {
            return Err(anyhow!("Header bytes invalid"));
        }

        let remaining_bytes = &bytes[8..]; 
        let mut start = 8;
        let mut chunks: Vec<Chunk> = vec![];

        while (start <= remaining_bytes.len()) {
            println!("<<<<<");
            println!("NEW START {:?}", start);
            println!("REMAINING {:?}", remaining_bytes.len());
            println!("<<<<<");

            // try result?
            let (sub_chunk, end) = Png::extract_chunk_from_bytes(start, bytes);
            chunks.push(sub_chunk);
            start =  end;
        }

        // TODO fix
        return Ok(Png{chunks: chunks});
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use crate::chunk::Chunk;
    use std::str::FromStr;
    use std::convert::TryFrom;

    #[test]
    fn test_from_chunks() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data_one: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();


        let combined: Vec<u8> = chunk_data_one.iter().chain(chunk_data_one.iter()).cloned().collect();
        let with_header: Vec<u8> = Png::STANDARD_HEADER.iter().chain(combined.iter()).cloned().collect();
        let combined_res = Png::try_from(with_header.as_ref()).unwrap();
        println!("{:?}", combined_res.to_string());
    }

}