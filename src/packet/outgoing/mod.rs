
mod login_message;
mod set_account_info;

use anyhow::Error;

use crate::util::writer::Writer;

#[derive(Debug)]
pub struct LoginMessage(pub LoginMessageKind);

#[derive(Debug, Clone)]
pub enum LoginMessageKind {
    FullClients = 5001,
    InvalidId = 7001,
    NoResponse = 7003,
    NoUserInfo = 7005,
    Banned = 7007,
    Online = 7010,
    FullRoom = 7011,
    FullRooms = 7012,
    NoRoom = 7013,
    LackOfLevel = 7014,
    UnbalancedTeamNumber = 7016,
    BoardNotReady = 7021,
    SameNickname = 7027,
    LackOfTicketForNameChange = 7028,
    DuplicatedNickname = 7029,
    InvalidAccountInfo = 8003,
}

#[derive(Debug)]
pub struct AccountInfo;

#[derive(Debug)]
pub struct ServerStatusResponse {
    pub code: u8,
}
impl OutPacketBuildable for ServerStatusResponse {
    fn opcode (&self) -> u8 { 2 }
    fn try_build(&self, writer: &mut Writer) -> Result<(), Error> {
        // FIXME
        let avail = 200;
        let max = 200;
        writer.write_u8(self.code)?;
        writer.write_u8(1)?;      // the number of servers
        writer.write_u16(401)?;   // (1) server uid
        writer.write_u16(avail)?; // (2) available
        writer.write_u16(max)?;   // (3) the max number of clients
        Ok(())
    }
}

pub trait OutPacketBuildable: Sync + Send + core::fmt::Debug {
    fn opcode (&self) -> u8;
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()>;
}


