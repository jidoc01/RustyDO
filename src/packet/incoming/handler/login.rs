use crate::{packet::incoming::InPacket, util::reader::Reader};
use super::InPacketHandler;

pub struct LoginHandler;

impl InPacketHandler for LoginHandler {
    fn opcode(&self) -> u8 { 3 }

    fn parse(&self, reader: &mut Reader) -> anyhow::Result<InPacket> {
        todo!()
    }
}