// Copyright 2022 JungHyun Kim
// This file is part of Foobar.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

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