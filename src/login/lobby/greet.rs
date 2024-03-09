
use evenio::handler::Local;

use crate::login::*;

use super::cache::LobbyCache;

/// Note that this is not handling events (see the definition of receiver).
pub fn handle_enter_lobby(
    receiver: Receiver<Insert<ClientOnLobby>>,
    mut cache: Local<LobbyCache>,
) {
    let e = receiver.event.entity;
    // 1. add the newcomer to the user list of the lobby.
    {
        cache.add_user(e);
    }

    // 2. send necessary packets to the newcomer.
    {
        todo!();
    }
}

pub fn handle_leave_lobby(
    receiver: Receiver<Remove<ClientOnLobby>>,
    mut cache: Local<LobbyCache>,
) {
    let e = receiver.event.entity;
    {
        cache.remove_user(e);
    }
}