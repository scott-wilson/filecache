extern crate data_encoding;

use std::fs::{File};
use std::path::Path;
use std::io::{BufReader, Error};
use std::io::prelude::*;
use self::data_encoding::{HEXLOWER};
use super::digestutils::{calculate_digest};

pub const MAX_CHUNK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Chunk {
    _id: String,
    _size: usize,
    _buffer: Vec<u8>,
}

impl Chunk {
    pub fn from_buffer(buffer: &Vec<u8>) -> Chunk {
        let mut inner_buffer = buffer.clone();
        inner_buffer.resize(MAX_CHUNK_SIZE, 0);

        let digest = calculate_digest(&inner_buffer);
        let id = HEXLOWER.encode(digest.as_ref());

        return Chunk {_id: id, _size: buffer.len(), _buffer: inner_buffer};
    }

    pub fn from_cache(id: &String, cache_path: &Path) -> Result<Chunk, Error> {
        let path = Path::new(cache_path).join(&id);
        let file = File::open(&path)?;
        let mut reader = BufReader::new(&file);
        let mut buffer: Vec<u8> = Vec::with_capacity(MAX_CHUNK_SIZE);
        buffer.resize(MAX_CHUNK_SIZE, 0);
        reader.read(&mut buffer)?;

        return Ok(Chunk {_id: id.clone(), _size: MAX_CHUNK_SIZE, _buffer: buffer});
    }

}

impl Chunk {
    pub fn id(&self) -> &String {&self._id}
    pub fn size(&self) -> &usize {&self._size}
    pub fn buffer(&self) -> &Vec<u8> {&self._buffer}
}

impl Chunk {
    pub fn write_to_cache(&self, path: &Path) -> Result<(), Error> {
        let path = Path::new(&path).join(&self._id);
        let mut file = File::create(&path)?;
        file.write_all(&self._buffer)?;

        return Ok(());
    }
}
