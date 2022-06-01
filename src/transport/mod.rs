//! Transports for NETCONF

use std::io;

pub mod ssh;

/// Trait for NETCONF transport
pub trait Transport: Send {
    fn read(&mut self) -> io::Result<String>;
    fn write(&mut self, data: &str) -> io::Result<()>;
}
