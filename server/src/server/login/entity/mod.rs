pub mod room;
pub mod timer;
pub mod session;

use std::net::SocketAddr;

use super::{component::*, conn::{MsgToConnSender, MsgToConn}};
use crate::prelude::*;

#[derive(PartialEq)]
pub enum EntityKind {
    Client,
    Room,
    Timer,
}