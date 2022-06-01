// We need to have the Read/Write traits into our scope
// so that we can make use of the methods they define.
use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use xml::writer::XmlEvent;
use xml::{EmitterConfig, EventWriter};

use crate::args::Args;

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

        let mut xml = Vec::new();
        let mut w = EmitterConfig::default()
            .perform_indent(true)
            .create_writer(&mut xml);
        //let _ = w.write(buffer.as_writer_event());
        let _ = handle_event(&mut w, buffer.to_s);

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

fn handle_event<W: Write>(w: &mut EventWriter<W>, line: String) -> Result<()> {
    let line = line.trim();
    let event: XmlEvent = if line.starts_with("+") && line.len() > 1 {
        XmlEvent::start_element(&line[1..]).into()
    } else if line.starts_with("-") {
        XmlEvent::end_element().into()
    } else {
        XmlEvent::characters(&line).into()
    };
    w.write(event)
}
