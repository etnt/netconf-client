// We need to have the Read/Write traits into our scope
// so that we can make use of the methods they define.
use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        let buffer = write_rx.recv().unwrap_or_default();
        if buffer.is_empty() {
            break;
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // "stop the program cleanly"
                return Ok(());
            }
            return Err(e);
        }
    }
    Ok(())
}
