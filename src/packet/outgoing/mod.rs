
mod login_message;
mod set_enc_data;
mod set_account_info;

use std::collections::HashMap;

use anyhow::Error;

use crate::util::writer::Writer;

pub struct LoginMessage(pub LoginMessageKind);
pub enum LoginMessageKind {
    NoId,
    InvalidInfo,
    AlreadyOnline,
    Banned,
}

pub struct SetEncData {
}

pub struct SetAccountInfo;


pub trait OutPacketBuildable: Sync + Send {
    fn try_build(&self, writer: &mut Writer) -> anyhow::Result<()>;
}


