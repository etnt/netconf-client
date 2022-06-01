use crossbeam::channel::unbounded;
use env_logger::{Builder, Target};
use log::debug;
use mypipe::{args::Args, read, session, write};
use std::io::Result;
use std::thread::{self};

fn main() -> Result<()> {
    let args = Args::parse();
    let args2 = args.clone();
    let infile = args.infile;
    let outfile = args.outfile;
    let log_level = args.log_level;
    let mut elog = Builder::from_default_env();
    elog.target(Target::Stderr).filter(None, log_level).init();
    debug!("get-config: {}", args.get_config);

    let (session_tx, session_rx) = unbounded();
    let (write_tx, write_rx) = unbounded();

    // When asking for the complete config we don't expect any input.
    if !args.get_config {
        let read_handle = thread::spawn(move || read::read_loop(&infile, session_tx));
        let read_io_result = read_handle.join().unwrap();
        read_io_result?;
    };
    let session_handle = thread::spawn(move || session::session_loop(args2, session_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed
    // .join() returns a 'thread::Result<io::Result<()>>'
    let session_io_result = session_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // return if any threads returned an error
    session_io_result?;
    write_io_result?;

    Ok(())
}
