extern crate data_encoding;

use std::cmp;
use std::io::prelude::*;
use std::io::{BufReader, Error, SeekFrom};
use std::path::Path;
use std::fs::{File};
use self::data_encoding::{HEXLOWER};
use super::digestutils::{calculate_file_digest};
use super::chunk::{Chunk, MAX_CHUNK_SIZE};

pub struct Bundle<'a> {
    _id: String,
    _path: &'a Path,
    _size: usize,
    _chunks: Vec<Chunk>,
}

impl<'a> Bundle<'a> {
    pub fn from_path(path: &Path) -> Result<Bundle, Error> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let digest = calculate_file_digest(&mut reader)?;
        reader.seek(SeekFrom::Start(0))?;
        let (chunks, size) = get_chunks(&mut reader)?;

        return Ok(Bundle{
            _id: HEXLOWER.encode(digest.as_ref()),
            _path: path.clone(),
            _chunks: chunks,
            _size: size,
        });
    }
    pub fn from_ids(ids: Vec<String>, file_id: String, size: usize, cache_path: &Path) -> Result<Bundle, Error> {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(ids.len());

        for id in ids.iter() {
            let chunk = Chunk::from_cache(id, cache_path)?;
            chunks.push(chunk);
        }

        return Ok(Bundle {
            _id: file_id,
            _path: cache_path,
            _chunks: chunks,
            _size: size,
            });
    }
}

impl<'a> Bundle<'a> {
    pub fn id(&self) -> &String {&self._id}
    pub fn path(&self) -> &Path {&self._path}
    pub fn size(&self) -> &usize {&self._size}
    pub fn chunks(&self) -> &Vec<Chunk> {&self._chunks}
}

impl<'a> Bundle<'a> {
    pub fn write_to_cache(&self, path: &Path) -> Result<(), Error> {
        for chunk in self._chunks.iter() {
            chunk.write_to_cache(&path)?;
        }
        return Ok(());
    }
    pub fn write_to_path(&self, path: &Path) -> Result<(), Error> {
        let mut file = File::create(&path)?;
        let mut total_size: usize = self.size().clone();

        for chunk in self._chunks.iter() {
            let chunk_size = cmp::min(total_size, *chunk.size());
            file.write(&chunk.buffer()[..chunk_size])?;
            total_size -= chunk_size;
        }

        return Ok(());
    }
}

fn get_chunks(reader: &mut Read) -> Result<(Vec<Chunk>, usize), Error> {
    let mut buffer: Vec<u8> = Vec::with_capacity(MAX_CHUNK_SIZE);
    buffer.resize(MAX_CHUNK_SIZE, 0);
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut chunk_size: usize;
    let mut size: usize = 0;

    loop {
        chunk_size = reader.read(&mut buffer)?;
        size += chunk_size;

        if chunk_size == 0 {
            break;
        }

        chunks.push(Chunk::from_buffer(&buffer));
    }

    return Ok((chunks, size));
}
