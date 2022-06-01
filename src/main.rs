use crossbeam::channel::unbounded;
use env_logger::{Builder, Target};
use log::debug;
use mypipe::{args::Args, read, session, write};
use std::io::Result;
use std::thread::{self};

fn main() -> Result<()> {
    let args = Args::parse();
    let args2 = args.clone(); // FIXME these clones looks funny
    let args3 = args.clone();
    let args4 = args.clone();
    let log_level = args.log_level;
    let mut elog = Builder::from_default_env();
    elog.target(Target::Stderr).filter(None, log_level).init();

    let (session_tx, session_rx) = unbounded();
    let (write_tx, write_rx) = unbounded();

    // When asking for the complete config we don't expect any input.
    let read_handle = thread::spawn(move || read::read_loop(args2, session_tx));
    let session_handle = thread::spawn(move || {
        session::session_loop(args3, session_rx, write_tx)
    });
    let write_handle =
        thread::spawn(move || write::write_loop(args4, write_rx));

    // crash if any threads have crashed
    // .join() returns a 'thread::Result<io::Result<()>>'
    let read_io_result = read_handle.join().unwrap();
    let session_io_result = session_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // return if any threads returned an error
    read_io_result?;
    session_io_result?;
    write_io_result?;

    Ok(())
}
