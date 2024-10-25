use crate::chunk::Chunk;

const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
 
struct png {
    chunks: Vec<Chunk>
}