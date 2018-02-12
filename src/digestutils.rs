use std::io::Error;
use std::io::prelude::*;
use ring::digest::{Context, Digest, SHA256};

pub fn calculate_digest(buffer: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(buffer);
    context.finish()
}

pub fn calculate_file_digest(reader: &mut Read) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = Vec::with_capacity(1024);
    let mut count: usize;

    loop {
        count = reader.read(&mut buffer)?;

        if count == 0 {
            break
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
