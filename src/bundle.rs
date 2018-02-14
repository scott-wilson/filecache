use std::cmp;
use std::io::prelude::*;
use std::io::{BufReader, Error, SeekFrom};
use std::path::Path;
use std::fs::File;
use data_encoding::HEXLOWER;
use super::digestutils::calculate_file_digest;
use super::chunk::{Chunk, MAX_CHUNK_SIZE};

pub struct Bundle<'a> {
    id: String,
    path: &'a Path,
    size: usize,
    chunks: Vec<Chunk>,
}

impl<'a> Bundle<'a> {
    pub fn from_path(path: &Path) -> Result<Bundle, Error> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let digest = calculate_file_digest(&mut reader)?;
        reader.seek(SeekFrom::Start(0))?;
        let (chunks, size) = get_chunks(&mut reader)?;

        Ok(Bundle{
            id: HEXLOWER.encode(digest.as_ref()),
            path: path,
            chunks: chunks,
            size: size,
        })
    }
    pub fn from_ids(ids: &[String], file_id: String, size: usize, cache_path: &'a Path) -> Result<Bundle<'a>, Error> {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(ids.len());

        for id in ids.iter() {
            let chunk = Chunk::from_cache(id, cache_path)?;
            chunks.push(chunk);
        }

        Ok(Bundle {
            id: file_id,
            path: cache_path,
            chunks: chunks,
            size: size,
        })
    }
}

impl<'a> Bundle<'a> {
    pub fn id(&self) -> &String {&self.id}
    pub fn path(&self) -> &Path {self.path}
    pub fn size(&self) -> usize {self.size}
    pub fn chunks(&self) -> &Vec<Chunk> {&self.chunks}
}

impl<'a> Bundle<'a> {
    pub fn write_to_cache<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        for chunk in &self.chunks {
            chunk.write_to_cache(path.as_ref())?;
        }
        Ok(())
    }
    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = File::create(path.as_ref())?;
        let mut total_size = self.size();

        for chunk in &self.chunks {
            let chunk_size = cmp::min(total_size, chunk.size());
            file.write_all(&chunk.buffer()[..chunk_size])?;
            total_size -= chunk_size;
        }

        Ok(())
    }
}

fn get_chunks<R: Read>(reader: &mut R) -> Result<(Vec<Chunk>, usize), Error> {
    let mut buffer = Vec::with_capacity(MAX_CHUNK_SIZE);
    buffer.resize(MAX_CHUNK_SIZE, 0);
    let mut chunks = Vec::new();
    let mut chunk_size: usize;
    let mut size = 0;

    loop {
        chunk_size = reader.read(&mut buffer)?;
        size += chunk_size;

        if chunk_size == 0 {
            break;
        }

        chunks.push(Chunk::from_buffer(&buffer));
    }

    Ok((chunks, size))
}
