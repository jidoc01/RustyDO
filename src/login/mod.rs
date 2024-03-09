mod component;
mod bulletin;
mod lobby;
mod room;
mod game;
mod net;
mod event;
mod greet;
mod packet;

pub use component::*;
pub use crate::prelude::*;

use crate::{packet::{incoming::InPacket, outgoing::{OutPacketBuildable, }}, world::WorldHelper};

/// A message to be sent to a client.
#[derive(Debug)]
pub enum ClientJob {
    OnReceive(InPacket),
    OnDisconnected
}

/// A message to be sent to a client session.
pub enum ClientSessionJob {
    SendPacket(Arc<dyn OutPacketBuildable + Send + Sync>),
    Disconnect,
}

pub fn init(world_helper: &mut WorldHelper) {
    net::init(world_helper);
    event::init(world_helper);
    greet::init(world_helper);
    bulletin::init(world_helper);
}
