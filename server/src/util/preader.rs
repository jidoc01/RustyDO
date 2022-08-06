use crate::prelude::*;
use super::*;

use encoding::{all, Encoding, DecoderTrap};

impl PacketReader {
    pub fn new(body: &[u8]) -> Self {
        let opcode = body[0];
        let data = Vec::from(body);
        PacketReader {
            opcode: opcode,
            data: data,
            offset: 8 /* Skip the header part in 'body'. */
        }
    }

    pub fn left(&self) -> usize {
        self.len() - self.offset
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn seek(&mut self, delta: usize) {
        self.offset = self.offset + delta;
    }

    pub fn u8(&mut self) -> u8 {
        let ret = self.data[self.offset];
        self.seek(1);
        ret
    }

    pub fn u16(&mut self) -> u16 {
        let ret = read_u16(&self.data, self.offset);
        self.seek(2);
        ret
    }

    pub fn u32(&mut self) -> u32 {
        let ret = read_u32(&self.data, self.offset);
        self.seek(4);
        ret
    }

    pub fn string(&mut self, len: usize) -> String {
        let euc_kr = all::WINDOWS_949;
        let data = self.vec(len);
        let actual_len = {
            let mut l = len;
            for i in 0..len {
                if data[i] == 0 {
                    l = i;
                    break;
                }
            }
            l
        };
        euc_kr.decode(&data[..actual_len], DecoderTrap::Ignore).unwrap()
    }

    /// It's used when (1) it contains a string with a null terminator and
    /// (2) its actual length is not specified.
    pub fn string_with_null(&mut self) -> Result<String> {
        // Get the actual length until null.
        let mut off = self.offset;
        let data_len = self.data.len();
        loop {
            if off >= data_len {
                bail!("Invalid input: no terminator");
            } else {
                if self.data[off] == 0 {
                    break;
                } else {
                    off += 1;
                }
            }
        }
        Ok(self.string(off + 1)) // Including an null terminator.
    }

    pub fn vec(&mut self, len: usize) -> Vec<u8> {
        let ret = Vec::from(&self.data[self.offset .. self.offset + len]);
        self.seek(len);
        ret
    }

    pub fn to_str(&self) -> String {
        format!("{:x?}", self.data)
    }
}