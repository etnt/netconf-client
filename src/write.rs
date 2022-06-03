// We need to have the Read/Write traits into our scope
// so that we can make use of the methods they define.
use crate::args::Args;
use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use xml::{reader::ParserConfig, writer::EmitterConfig};

pub fn write_loop(args: Args, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let outfile = &args.outfile;
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

        // FIXME not working atm.
        // See: https://users.rust-lang.org/t/pretty-printing-xml/76372/6
        //
        let mut xml = format_xml(&buffer).unwrap_or_default().as_bytes();

        if let Err(e) = writer.write_all(&xml) {
            if e.kind() == ErrorKind::BrokenPipe {
                // "stop the program cleanly"
                return Ok(());
            }
            return Err(e);
        }
    }
    Ok(())
}

fn format_xml(src: &[u8]) -> Result<String> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(&mut dest);
    for event in reader {
        if let Some(event) = event?.as_writer_event() {
            writer.write(event).unwrap();
        }
    }
    Ok(String::from_utf8(dest).unwrap())
}
