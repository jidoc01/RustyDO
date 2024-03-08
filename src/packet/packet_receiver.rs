use crate::encrypt::decrypt_header;

use super::incoming::InPacket;

use crate::prelude::*;

/// A threshold of incoming body length.
/// TODO: decide a proper value.
const INCOMING_BODY_LEN_THRESHOLD: usize = 2048;

#[derive(Default)]
pub struct PacketReceiver {
    gathering: Vec<u8>,
}

impl PacketReceiver {
    pub fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        self.gathering.extend(data.as_ref());
    }

    pub fn clear(&mut self) {
        self.gathering.clear();
    }

    pub fn try_fetch_body(&mut self) -> anyhow::Result<Option<Vec<u8>>> {
        let mut gathering = &mut self.gathering;

        let header = {
            if gathering.len() < HEADER_SIZE {
                return Ok(None);
            }
            let mut header = gathering[..HEADER_SIZE].to_vec();
            decrypt_header(&mut header);
            header
        };

        let body_len = {
            let temp_bytes = [header[0], header[1]];
            u16::from_le_bytes(temp_bytes) as usize
        };
        if body_len >= INCOMING_BODY_LEN_THRESHOLD {
            anyhow::bail!("Invalid body length: {}", body_len);
        }
        let chunk_size = HEADER_SIZE + body_len + TAIL_SIZE;
        if gathering.len() < chunk_size {
            return Ok(None);
        }

        let body = gathering[HEADER_SIZE..HEADER_SIZE + body_len].to_vec();
        gathering.drain(..chunk_size);
        Ok(Some(body))
    }
}