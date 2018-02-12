use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Error};
use std::io::prelude::*;
use data_encoding::HEXLOWER;
use super::digestutils::calculate_digest;

pub const MAX_CHUNK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Chunk {
    id: String,
    size: usize,
    buffer: Vec<u8>,
}

impl Chunk {
    pub fn from_buffer(buffer: &[u8]) -> Chunk {
        let inner_buffer = vec![0u8; MAX_CHUNK_SIZE];
        let digest = calculate_digest(&inner_buffer);
        let id = HEXLOWER.encode(digest.as_ref());

        Chunk {id: id, size: buffer.len(), buffer: inner_buffer}
    }

    pub fn from_cache(id: &str, cache_path: &Path) -> Result<Chunk, Error> {
        let path = Path::new(cache_path).join(&id);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(&file);
        let mut buffer = Vec::with_capacity(MAX_CHUNK_SIZE);
        buffer.resize(MAX_CHUNK_SIZE, 0);
        reader.read_exact(&mut buffer)?;

        Ok(Chunk {id: id.to_string(), size: MAX_CHUNK_SIZE, buffer: buffer})
    }

}

impl Chunk {
    pub fn id(&self) -> &String {&self.id}
    pub fn size(&self) -> usize {self.size}
    pub fn buffer(&self) -> &Vec<u8> {&self.buffer}
}

impl Chunk {
    pub fn write_to_cache(&self, path: &Path) -> Result<(), Error> {
        let path = path.join(&self.id);
        let mut file = File::create(&path)?;
        file.write_all(&self.buffer)?;

        Ok(())
    }
}
