mod login;
mod disconnect;
mod enter_lobby;

use crate::*;

use self::{disconnect::{handle_disconnect_event_before_login, handle_disconnect_event_in_bulletin, handle_disconnect_event_in_lobby, handle_when_disconnecting}, login::handle_login_event};

#[derive(Event)]
pub struct LoginEvent {
    #[event(target)]
    pub entity: EntityId,
    pub id: String,
    pub pw: String
}

#[derive(Event)]
struct DisconnectEvent {
    #[event(target)]
    pub entity: EntityId,
}

#[derive(Event)]
pub struct EnterLobbyEvent {
    #[event(target)]
    pub entity: EntityId,
}

pub fn init(world_helper: &mut WorldHelper) {
    world_helper
        .add_event::<LoginEvent>()
        .add_event::<DisconnectEvent>()
        .add_event::<EnterLobbyEvent>();
    world_helper
        .add_system(handle_login_event)
        .add_system(handle_when_disconnecting)
        .add_system(handle_disconnect_event_before_login)
        .add_system(handle_disconnect_event_in_bulletin)
        .add_system(handle_disconnect_event_in_lobby)
    ;
}