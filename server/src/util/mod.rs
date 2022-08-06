mod preader;
mod pwriter;
mod object;
pub mod db;
mod config;
pub mod hash;

pub use object::*;
pub use db::*;
pub use crate::util::config::Config;

#[derive(Debug)]
pub struct PacketReader {
    opcode: u8,
    data: Vec<u8>,
    offset: usize
}

pub struct PacketWriter {
    opcode: u8,
    data: Vec<u8>,
}
