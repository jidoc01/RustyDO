use crate::{packet::incoming::InPacket, util::reader::Reader};
use super::InPacketHandler;

pub struct RequestNoticeHandler;

impl InPacketHandler for RequestNoticeHandler {
    fn opcode(&self) -> u8 { 3 }

    fn parse(&self, reader: &mut Reader) -> anyhow::Result<InPacket> {
        todo!()
    }
}
