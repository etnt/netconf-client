use crate::args::Args;
use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read_loop(args: Args, session_tx: Sender<Vec<u8>>) -> Result<()> {
    if args.get_config {
        return Ok(());
    };

    // Box is a Smart Pointer with a fixed size which places
    // its value on the heap. In this case, the value is anything
    // that satisfies the Read Trait.
    let infile = &args.infile;
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        if session_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }
    let _ = session_tx.send(Vec::new());
    Ok(())
}
