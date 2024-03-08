mod login;
mod request_notice;

use crate::*;

use crate::util::reader::Reader;

use self::{login::LoginHandler, request_notice::RequestNoticeHandler};

use super::InPacket;

macro_rules! register_handlers {
    ($name:ident, [$($i:ident),*]) => {
        lazy_static! {
            static ref $name: HandlerMap = {
                let mut mapping = HandlerMap::default();
                $({ mapping.register_handler($i); })*
                mapping
            };
        }
    };
}

register_handlers!(HANDLER_MAP, [
    LoginHandler,
    RequestNoticeHandler
]);

pub trait InPacketHandler {
    fn opcode(&self) -> u8;
    fn parse(&self, reader: &mut Reader) -> anyhow::Result<InPacket>;
}

pub struct HandlerMap(HashMap<u8, Box<dyn InPacketHandler + Send + Sync>>);

impl Default for HandlerMap {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl HandlerMap {
    fn parse(&self, opcode: u8, reader: &mut Reader) -> anyhow::Result<InPacket> {
        match self.0.get(&opcode) {
            Some(handler) => handler.parse(reader),
            None => anyhow::bail!("unknown packet opcode: {}", opcode)
        }
    }
}

impl HandlerMap {
    fn register_handler<T: InPacketHandler + 'static + Send + Sync>(&mut self, handler: T) {
        self.0.insert(handler.opcode(), Box::new(handler));
    }
}

pub fn try_parse(reader: &mut Reader) -> anyhow::Result<InPacket> {
    let opcode = reader.read_u8()?;
    reader.advance(7);
    HANDLER_MAP.parse(opcode, reader)
}
