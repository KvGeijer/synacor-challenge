mod interpreter;

use std::fs;
use std::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

use interpreter::interpret;

fn main() {
    let code = parse("challenge.bin");

    // interpret(&[9, 32768, 32769, 4, 19, 32768]);
    interpret(&code);
}

// Very ugly, but could not manage to read it directly into the vec with this method?
fn parse(path: &str) -> Vec<u16> {
    // let nbr_bytes = fs::metadata(path).unwrap().len() as usize;
    // let mut code = Vec::with_capacity(nbr_bytes / 2);
    const NBR_BYTES: usize = 60100;
    let mut code = [0; NBR_BYTES / 2];
    let file = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    reader
        .read_u16_into::<LittleEndian>(&mut code)
        .expect("Could not read as u16 :()");

    Vec::from_iter(code.into_iter())
}
