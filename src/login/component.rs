use crate::{login::*, storage::account::Account};

use super::{ClientJob, ClientSessionJob};

#[derive(Component)]
pub struct ClientJobReceiver(pub UnboundedReceiver<ClientJob>);

#[derive(Component)]
pub struct ClientSessionJobSender(pub UnboundedSender<ClientSessionJob>);

#[derive(Component)]
pub struct ClientAddr(pub SocketAddr);

#[derive(Component, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub struct ClientUid(pub u16);

#[derive(Component, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub struct ClientId(pub String);

#[derive(Component)]
pub struct ClientName(pub String);

#[derive(Component)]
pub struct ClientLevel(pub u32);

#[derive(Component)]
pub struct ClientAccount(pub Account);

#[derive(Component)]
pub struct ClientDisconnecting;

#[derive(Component)]
pub struct ClientOnBulletinBoard;

#[derive(Component)]
pub struct ClientOnLobby;

#[derive(Component)]
pub struct ClientOnRoom;

#[derive(Component)]
pub struct ClientOnGame;

impl ClientSessionJobSender {
    pub fn send(&self, msg: ClientSessionJob) {
        self.0.send(msg).ignore();
    }

    pub fn send_packet<T: OutPacketBuildable + 'static>(&self, pkt: T) {
        self.send(ClientSessionJob::SendPacket(Arc::new(pkt)));
    }

    pub fn send_shared_packet(&self, pkt: Arc<dyn OutPacketBuildable + Send + Sync>) {
        self.send(ClientSessionJob::SendPacket(pkt));
    }
}
