use std::borrow::Borrow;

use crate::{login::*, storage::{account::Account, Storage, }};

use super::DisconnectEvent;

pub fn handle_when_disconnecting (
    receiver: Receiver<Insert<ClientDisconnecting>, &mut ClientJobReceiver>,
    mut sender: Sender<DisconnectEvent>,
) {
    debug!("handle_when_disconnecting");
    let e = receiver.event.entity;
    sender.send(DisconnectEvent { entity: e });
    receiver.query.0.close();
}

pub fn handle_disconnect_event_before_login (
    receiver: Receiver<DisconnectEvent, Not<&ClientAccount>>,
    mut despawner: Sender<Despawn>,
) {
    debug!("handle_disconnect_event_before_login");
    let e = receiver.event.entity;
    despawner.despawn(e);
    debug!("despawned entity {:?}", e);
}

pub fn handle_disconnect_event_in_bulletin (
    receiver: Receiver<DisconnectEvent, (&ClientAccount, &ClientOnBulletinBoard)>,
    mut despawner: Sender<Despawn>,
    Single(storage): Single<&mut Storage>,
) {
    let e = receiver.event.entity;
    let account = &receiver.query.0.0;
    let filter = doc! { "id": account.id.clone() };
    storage.update_one_with_replacement::<Account>(filter, account.clone());
    despawner.despawn(e);
}

pub fn handle_disconnect_event_in_lobby(
    receiver: Receiver<DisconnectEvent, &ClientOnLobby>,
    mut sender: Sender<(Remove<ClientOnLobby>, Insert<ClientOnBulletinBoard>, DisconnectEvent)>,
) {
    let e = receiver.event.entity;
    sender.remove::<ClientOnLobby>(e);
    sender.insert(e, ClientOnBulletinBoard);
    sender.send(DisconnectEvent { entity: e }); // re-send the disconnect event
}