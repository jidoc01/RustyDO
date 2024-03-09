use crate::login::*;

use super::EnterLobbyEvent;

/// From the bulletin board, we let the client to enter the lobby.
pub fn handle_enter_lobby_event_from_bulletin(
    receiver: Receiver<EnterLobbyEvent, &ClientOnBulletinBoard>,
    mut remover: Sender<Remove<ClientOnBulletinBoard>>,
    mut adder: Sender<Insert<ClientOnLobby>>,
) {
    let e = receiver.event.entity;
    remover.remove::<ClientOnBulletinBoard>(e);
    adder.insert(e, ClientOnLobby);
}

