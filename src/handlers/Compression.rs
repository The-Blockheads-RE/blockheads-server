use std::io::{Cursor, Error, Read, Write};
use libflate::gzip::{Decoder, Encoder};

pub struct GZIP {}

impl GZIP {
    pub fn encode(bytes: &Vec<u8>) -> Result<Vec<u8>, Error> {
        let mut encoder = Encoder::new(Vec::new()).unwrap();
        encoder.write_all(&bytes).expect("Failed to write bytes!");
        return encoder.finish().into_result();
    }
    pub fn decode(bytes: &Vec<u8>) -> Result<Vec<u8>, Error> {
        let curs = Cursor::new(&bytes[0..]);
        let mut decoder = match Decoder::new(curs) {
            Ok(res) => res,
            Err(e) => return Err(e)
        };
        let mut result = Vec::new();
        return match decoder.read_to_end(&mut result) {
            Ok(_) => Ok(result),
            Err(e) => Err(e)
        };
    }
}