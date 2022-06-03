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
        };

        if args.pretty_print {
            // See: https://users.rust-lang.org/t/pretty-printing-xml/76372/6
            if let Err(error) = to_writer_pretty(&mut writer, &buffer) {
                return Ok(());
            }
        } else {
            if let Err(e) = writer.write_all(&buffer) {
                if e.kind() == ErrorKind::BrokenPipe {
                    // "stop the program cleanly"
                    return Ok(());
                }
                return Err(e);
            }
        }
    }

    Ok(())
}

pub fn to_writer_pretty<W>(writer: &mut W, buf: &[u8]) -> std::io::Result<usize>
where
    W: std::io::Write,
{
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(buf);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .autopad_comments(false)
        .create_writer(writer);
    for event in reader {
        if let Some(event) = event.map_err(to_io)?.as_writer_event() {
            writer.write(event).map_err(to_io)?;
        }
    }
    Ok(buf.len())
}

fn to_io<E>(e: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    std::io::Error::new(std::io::ErrorKind::Other, e)
}
