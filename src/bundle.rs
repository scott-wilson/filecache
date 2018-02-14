use std::cmp;
use std::io::prelude::*;
use std::io::{BufReader, Error, SeekFrom};
use std::path::Path;
use std::fs::File;
use data_encoding::HEXLOWER;
use super::digestutils::calculate_file_digest;
use super::chunk::{Chunk, MAX_CHUNK_SIZE};

pub struct Bundle<P: AsRef<Path>> {
    id: String,
    path: P,
    size: usize,
    chunks: Vec<Chunk>,
}

impl<P: AsRef<Path>> Bundle<P> {
    pub fn from_path(path: P) -> Result<Bundle<P>, Error> {
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

    pub fn from_ids(ids: &[String], file_id: String, size: usize, cache_path: P) -> Result<Bundle<P>, Error> {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(ids.len());

        for id in ids.iter() {
            let chunk = Chunk::from_cache(id, &cache_path)?;
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

impl<P: AsRef<Path>> Bundle<P> {
    pub fn id(&self) -> &String {&self.id}
    pub fn path(&self) -> &Path {self.path.as_ref()}
    pub fn size(&self) -> usize {self.size}
    pub fn chunks(&self) -> &Vec<Chunk> {&self.chunks}
}

impl<P: AsRef<Path>> Bundle<P> {
    pub fn write_to_cache(&self, path: P) -> Result<(), Error> {
        for chunk in &self.chunks {
            chunk.write_to_cache(path.as_ref())?;
        }
        Ok(())
    }
    pub fn write_to_path(&self, path: P) -> Result<(), Error> {
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
