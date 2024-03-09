use crate::{packet::incoming::InPacket, util::reader::Reader};
use super::InPacketHandler;

pub struct LoginHandler;

impl InPacketHandler for LoginHandler {
    fn opcode(&self) -> u8 { 3 }

    fn parse(&self, reader: &mut Reader) -> anyhow::Result<InPacket> {
        let id = reader.read_fixed_string(12+1)?;
        let pw = reader.read_fixed_string(20+1)?;
        let pkt = InPacket::LoginRequest { id, pw };
        Ok(pkt)
    }
}