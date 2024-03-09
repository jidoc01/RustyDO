mod handler;

use anyhow::Error;

use crate::util::reader::Reader;

use self::handler::try_parse;

#[derive(Debug)]
pub enum InPacket {
    ServerStatusRequest {
        code: u8,
    },
    LoginRequest {
        id: String,
        pw: String,
    },
    EnteringLobby,
    NoticeRequest,
    Unknown(u8),
    ParsingError(Error),
}

impl InPacket {
    pub fn parse<T: AsRef<[u8]>>(body: T) -> InPacket {
        let mut reader = Reader::from_ref(body.as_ref());
        try_parse(&mut reader).unwrap_or_else(|e| InPacket::ParsingError(e))
    }
}

