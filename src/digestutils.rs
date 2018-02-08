extern crate ring;

use std::io::{Error};
use std::io::prelude::*;
use self::ring::digest::{Context, Digest, SHA256};

pub fn calculate_digest(buffer: &Vec<u8>) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(&buffer);
    return context.finish();
}

pub fn calculate_file_digest(reader: &mut Read) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    let mut count: usize;

    loop {
        count = reader.read(&mut buffer)?;

        if count == 0 {
            break
        }
        context.update(&buffer[..count]);
    }

    return Ok(context.finish());
}
