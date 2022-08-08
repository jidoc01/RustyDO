// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use crate::prelude::*;
use super::*;

use byteorder::WriteBytesExt;
use byteorder::LittleEndian;
use encoding::{all, Encoding, EncoderTrap};

const BODY_MAGIC_STAMP: u32 = 0x12345678;

/// A module for writing a body part of a packet.
impl PacketWriter {
    pub fn new(opcode: u8) -> Self {
        let mut pw = PacketWriter {
            opcode: opcode,
            data: vec!(),
        };
        pw
            .u8(opcode)
            .u8(0)  // Dummy
            .u16(0) // Dummy
            .u32(BODY_MAGIC_STAMP);
        pw
    }

    pub fn len(&mut self) -> usize {
        self.data.len()
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn u8(&mut self, v: u8) -> &mut Self {
        let _ = self.data.write_u8(v);
        self
    }

    pub fn u16(&mut self, v: u16) -> &mut Self {
        let _ = self.data.write_u16::<LittleEndian>(v);
        self
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        let _ = self.data.write_u32::<LittleEndian>(v);
        self
    }

    pub fn i32(&mut self, v: i32) -> &mut Self {
        let _ = self.data.write_i32::<LittleEndian>(v);
        self
    }

    pub fn string(&mut self, msg: &str, size: usize) -> &mut Self {
        let euc_kr = all::WINDOWS_949;
        match euc_kr.encode(msg, EncoderTrap::Ignore) {
            Ok(data) => {
                let len = data.len();
                if len > size {
                    self.u8s(&data[..size])
                }
                else {
                    let shortage = size - len;
                    self.u8s(&data);
                    self.vec(&vec![0u8; shortage])
                }
            },
            _ => {
                println!("Invalid input for encoding: {msg}");
                self.vec(&vec![0u8; size]) // Just fill with null terminators.
            }
        }
    }
    
    pub fn string_with_null(&mut self, msg: &str) -> &mut Self {
        let euc_kr = all::WINDOWS_949;
        match euc_kr.encode(msg, EncoderTrap::Ignore) {
            Ok(data) => {
                self.u8s(&data);
            },
            _ => {
                println!("Invalid input for encoding: {msg}");
            }
        }
        self.u8(0)
    }

    pub fn vec(&mut self, vec: &Vec<u8>) -> &mut Self {
        self.u8s(vec.as_ref())
    }
    
    pub fn u8s(&mut self, bytes: &[u8]) -> &mut Self {
        let data = &mut self.data;
        let len = data.len();
        let vec_len = bytes.len();
        let new_len = len + vec_len;
        data.resize(new_len, 0u8); // Extend its length.
        for i in 0..vec_len { // Append the vector.
            data[len + i] = bytes[i];
        }
        self
    }

    pub fn pad(&mut self, n: usize) -> &mut Self {
        self.vec(&vec![0u8; n])
    }

    /// Pad null bytes until its length reaches the given length.
    pub fn pad_to(&mut self, len: usize) -> &mut Self {
        let curr_len = self.data.len();
        assert!(curr_len <= len);
        self.pad(len - curr_len)
    }

    fn create_header(body_len: usize) -> Vec<u8> {
        let mut bytes = vec!();
        let random_seed = rand::random::<u8>() % 6 + 2;
        let _ = bytes.write_u16::<LittleEndian>(body_len as u16);
        let _ = bytes.write_u16::<LittleEndian>(0xb9);
        let _ = bytes.write_u16::<LittleEndian>(0x08);
        let _ = bytes.write_u16::<LittleEndian>(0x09);
        let _ = bytes.write_u8(random_seed);
        bytes
    }

    /// Encrypt and return the packet.
    /// NOTE: Do not call this frequently for the same packet.
    pub fn as_vec(&mut self) -> Vec<u8> {
        let body_len = self.len();
        let header = Self::create_header(body_len);
        let header_enc = crypt::encode(&header).unwrap();
        let body = &self.data;
        let body_enc = crypt::encrypt(body);
        let tail = vec![0u8; TAIL_SIZE];
        let list = [ &header_enc, &body_enc, &tail ];
        let pkt = concat_list_of_vec(&list);
        pkt
    }
}