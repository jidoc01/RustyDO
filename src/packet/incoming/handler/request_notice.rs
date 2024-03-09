use crate::{packet::incoming::InPacket, util::reader::Reader};
use super::InPacketHandler;

pub struct RequestNoticeHandler;

impl InPacketHandler for RequestNoticeHandler {
    fn opcode(&self) -> u8 { 255 } // FIXME

    fn parse(&self, _reader: &mut Reader) -> anyhow::Result<InPacket> {
        let pkt = InPacket::NoticeRequest;
        Ok(pkt)
    }
}
