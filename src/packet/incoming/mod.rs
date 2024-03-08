mod handler;

use anyhow::Error;

use crate::util::reader::Reader;

use self::handler::try_parse;

pub enum InPacket {
    RequestServerStatus,
    ParsingError(Error),
    Login {
        id: String,
        pw: String,
    },
    EnterLobby,
    RequestNotice,
}

impl InPacket {
    pub fn parse<T: AsRef<[u8]>>(body: T) -> InPacket {
        let mut reader = Reader::from_ref(body.as_ref());
        try_parse(&mut reader).unwrap_or_else(|e| InPacket::ParsingError(e))
    }
}
