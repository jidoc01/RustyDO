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

use crate::{packet::{incoming::InPacket, outgoing::{OutPacketBuildable, SetEncData}}, world::WorldHelper};

/// A message to be sent to a client.
pub enum ClientJob {
    OnReceive(InPacket),
    OnDisconnected
}

/// A message to be sent to a client session.
pub enum ClientSessionJob {
    SendPacket(Arc<dyn OutPacketBuildable + Send + Sync>),
    SetEncData(ClientEncData),
    Disconnect,
}

pub fn init(world_helper: &mut WorldHelper) {
    net::init(world_helper);
    event::init(world_helper);
    greet::init(world_helper);
    bulletin::init(world_helper);
}

fn greet_new_enc_data(
    receiver: Receiver<Insert<ClientEncData>, &ClientSessionJobSender>,
) {
    let enc_data = &receiver.event.component;
    let sender = &receiver.query.0;
    // TODO: just assign it when the client is connected from the client
    // session.
    sender.send(ClientSessionJob::SetEncData(enc_data.clone()));
    let pkt = SetEncData {
    };
    sender.send(ClientSessionJob::SendPacket(Arc::new(pkt)));
}

