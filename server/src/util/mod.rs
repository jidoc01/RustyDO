// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

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
