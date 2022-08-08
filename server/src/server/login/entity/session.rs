// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use super::*;

pub fn create(world: &mut World, addr: SocketAddr) -> &mut Entity {
    let id = world.push();
    let entity = world.get_mut(&id).unwrap();
    entity.push(EntityKind::Client);
    entity.push(addr);
    entity.push(ClientState::BeforeLogin);
    entity
}

pub fn send(entity: &Entity, pkt: Vec<u8>) {
    let _ = entity
        .get::<MsgToConnSender>()
        .unwrap()
        .send(MsgToConn::SendPacket(pkt));
}

pub fn send_by_eid(world: &World, eid: &EntityId, pkt: Vec<u8>) {
    let entity = world
        .get(eid)
        .unwrap();
    send(entity, pkt);
}