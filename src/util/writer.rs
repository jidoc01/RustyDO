use std::{io::Write, num::Wrapping};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::util::encoding::encode_to_vec;

const BUFFER_SIZE_THRESHOLD: usize = 1024;

pub struct Writer<'a> {
    buffer: &'a mut Vec<u8>,
}

impl<'a> Writer<'a> {
    pub fn from_vec_mut(value: &'a mut Vec<u8>) -> Self {
        Self {
            buffer: value
        }
    }
}

impl<'a> Writer<'a> {
    fn assert_space(&self, n: usize) -> anyhow::Result<()> {
        let required_len = self.buffer.len() + n;
        if required_len <= BUFFER_SIZE_THRESHOLD { Ok (()) }
        else { anyhow::bail!("too large data") }
    }

    pub fn write_u8(&mut self, v: u8) -> anyhow::Result<()> {
        self.assert_space(1)?;
        self.buffer.write_u8(v)?;
        Ok(())
    }

    pub fn write_u16(&mut self, v: u16) -> anyhow::Result<()> {
        self.assert_space(2)?;
        self.buffer.write_u16::<LittleEndian>(v.into())?;
        Ok(())
    }

    pub fn write_u32(&mut self, v: u32) -> anyhow::Result<()> {
        self.assert_space(4)?;
        self.buffer.write_u32::<LittleEndian>(v)?;
        Ok(())
    }

    pub fn write_bytes<T: AsRef<[u8]>>(&mut self, b: T) -> anyhow::Result<()> {
        let b = b.as_ref();
        let n = b.len();
        self.assert_space(n)?;
        self.buffer.write(b)?;
        Ok(())
    }

    pub fn read_fixed_string(&mut self, s: String, n: usize) -> anyhow::Result<()> {
        let b = encode_to_vec(s)?;
        anyhow::ensure!(b.len() <= n);
        self.write_bytes(&b)?;
        self.write_bytes(vec![0u8; n - b.len()])?; // TODO: do not allocate a new chunk
        Ok(())
    }

    pub fn get(&'a self) -> &'a[u8] {
        &self.buffer
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}
