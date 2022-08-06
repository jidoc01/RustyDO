pub mod before_login;
pub mod after_login;
pub mod on_lobby;
pub mod on_room;
pub mod on_game;

pub mod packet;

use super::*;

pub fn handle(server: &mut Server, entity_id: EntityId, pr: PacketReader) -> Result<()> {
    let state = server
        .world
        .get(&entity_id)?
        .get::<ClientState>()?;
    log!("[{}] Packet received: {}, {}", entity_id, pr.opcode(), pr.to_str());
    match state {
        ClientState::BeforeLogin => {
            before_login::handle(server, &entity_id, pr)
        },
        ClientState::AfterLogin => {
            after_login::handle(server, &entity_id, pr)
        },
        ClientState::OnLobby => {
            on_lobby::handle(server, &entity_id, pr)
        },
        ClientState::OnRoom => {
            on_room::handle(server, &entity_id, pr)
        },
        ClientState::OnGame => {
            on_game::handle(server, &entity_id, pr)
        }
    }
}