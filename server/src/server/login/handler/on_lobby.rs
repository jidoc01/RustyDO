// Copyright 2022 JungHyun Kim
// This file is part of Foobar.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use crate::{prelude::*, server::login::entity::room::TryAcceptClientResult};
use super::*;

const PACKET_CHAT: u8                   = 34;
const PACKET_ENTER_SHOP: u8             = 50;
const PACKET_GO_BACK: u8                = 54;
const PACKET_REQUEST_USER_LIST: u8      = 55;
const PACKET_REQUEST_ROOM_LIST: u8      = 57;
const PACKET_CREATE_ROOM: u8            = 59;
const PACKET_ENTER_ROOM: u8             = 62;

fn try_get_new_room_id(world: &mut World) -> Option<RoomId> {
    let id_set = {
        let mut id_set = HashSet::new();
        world
            .values()
            .iter()
            .for_each(|&entity| {
                match entity.get::<RoomInfo>() {
                    Ok(info) => { id_set.insert(info.room_uid); },
                    _ => {}
                }
            });
        id_set
    };

    for candidate in MIN_ROOM_ID..=MAX_ROOM_ID {
        if !id_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    return None;
}

// TODO: Do not make it public. Instead make an API to deal with on-chat requests.
pub fn on_chat(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    let unknown = pr.u8();
    let chat_kind = pr.u8(); // 0: Normal, 1: Whisper.
    let text = pr.string(193);

    let name = world
        .get(entity_id)?
        .get::<AfterLoginInfo>()?
        .user_schema
        .name
        .clone();
            
    if chat_kind == 1 { // Whisper.
        let receiver_name = pr.string(13);

        let users = world
            .select(|entity| {
                if let Ok(info) = entity.get::<AfterLoginInfo>() {
                    info.user_schema.name == receiver_name
                } else {
                    false
                }
            });
        
        if users.len() == 0 { // Could not find the user.
            // TODO: Send a packet to show an error message.
            let pkt = packet::chat(true, false, "", "존재하지 않는 아이디입니다.");
            let entity = world.get(entity_id).unwrap();
            entity::session::send(entity, pkt);
        } else {
            let receiver = users[0];
            let pkt = packet::chat(false, true, &name, &text);
            entity::session::send(receiver, pkt);
        }

        return Ok(());
    }

    let &state = world
        .get(entity_id)?
        .get::<ClientState>()?;

    let pkt = packet::chat(false, false, &name, &text);

    // TODO: Combine all together.
    // Just send the packet to different receivers with the same logic.
    match state {
        ClientState::OnLobby => {
            let users = world
                .select(|entity| {
                    if let Ok(ClientState::OnLobby) = entity.get::<ClientState>() {
                        true
                    } else {
                        false
                    }
                });

            for user in users {
                entity::session::send(user, pkt.clone());
            }
        },
        ClientState::OnRoom => {
            let users = {
                let entity = world.get(entity_id)?;
                let on_room_info = entity.get::<OnRoomInfo>()?;
                let room_entity_id = on_room_info.room_entity_id;
                let room_entity = world.get(&room_entity_id)?;
                let room_info = room_entity.get::<RoomInfo>()?;
                let mut users = vec!();
                room_info
                    .members
                    .iter()
                    .for_each(|member| {
                        if let Some(member_entity_id) = member {
                            let member_entity = world
                                .get(member_entity_id)
                                .unwrap();
                            users.push(member_entity);
                        }
                    });
                users
            };

            for user in users {
                entity::session::send(user, pkt.clone());
            }
        },
        ClientState::OnGame => {

        },
        _ => {}
    }

    Ok(())
}

fn enter_shop(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    // TODO

    Ok(())
}

fn go_back(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    server
        .world
        .get_mut(entity_id)?
        .push(ClientState::BeforeLogin);
    Ok(())
}

fn send_user_list(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    let entity = server.world.get(entity_id).unwrap();

    // Get a info list of users who are on lobby.
    let users = server
        .world
        .select(|entity| {
            if let Ok(state) = entity.get::<ClientState>() {
                match state {
                    ClientState::OnLobby => true,
                    _ => false
                }
            } else {
                false
            }
        });

    let user_count = users.len();

    let pkt = {
        let mut pw = PacketWriter::new(PACKET_REQUEST_USER_LIST + 1);
        pw
            .u16(user_count as u16)
            .pad_to(10);
        for user in users {
            let info = user
                .get::<AfterLoginInfo>()?;
            pw
                .string(&info.user_schema.name, 13)
                .u8(info.user_schema.level)
                .pad(4);
        }
        pw.as_vec()
    };

    entity::session::send(&entity, pkt);

    Ok(())
}

fn send_room_list(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    let is_waiting_only = if_else(pr.u8() == 1, true, false);
    let mode = pr.u8(); // 0: From the beginning, 1: >= (forward), 2: <= (backward)
    let base_room_id = pr.u8();
    
    let rooms = entity::room::get_room_list(world, is_waiting_only, mode, base_room_id);

    {
        let entity = world.get(entity_id).unwrap();
        let pkt = packet::room_list(world, &rooms);
        entity::session::send(&entity, pkt);
    }

    Ok(())
}

fn create_room(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    
    let room_id =
        match try_get_new_room_id(world) {
            Some(id) => id,
            None => { // Full of rooms.
                // TODO: Find an adequate packet.
                let entity = world.get(entity_id).unwrap();
                let pkt = packet::error(packet::ErrorKind::FullOfClientsInRoom);
                entity::session::send(entity, pkt);

                return Ok(());
            }
        };
    
    let name = pr.string(13);
	pr.seek(12);
    let password = pr.string(13);
    let max_users = pr.u8();
    let lev_condition = pr.u8();
    let lev_base = pr.u8();
	let bits = pr.u8();
    let allows_item = {
        if bits & (1 << 0) != 0 {
            true
        } else {
            false
        }
    };
    let allows_evol = {
        if bits & (1 << 1) != 0 {
            true
        } else {
            false
        }
    };

    // Create a new room into the world.
    let info = RoomInfo::new(room_id, &name, &password, *entity_id, allows_item, allows_evol, max_users, lev_condition, lev_base);
    let room = entity::room::create(world, info);
    
    // Insert into a room.
    /*
    match entity::room::try_accept_client(room, *entity_id) {
        TryAcceptClientResult::Ok => {},
        _ => panic!("A new room should be able to accept its master")
    }
    */

    let room_entity_id = room.id();

    let entity = world.get_mut(entity_id).unwrap();

    // Change entity's state.
    entity.push(ClientState::OnRoom);
    entity.push(OnRoomInfo::new(room_entity_id));

    // Send a packet to join the room.
    {
        let pkt =
            PacketWriter::new(PACKET_CREATE_ROOM + 1)
                .u16(room_id as u16)
                .as_vec();
        entity::session::send(entity, pkt);
    }

    Ok(())
}

fn enter_room(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    fn try_enter_room(world: &mut World, entity_id: &EntityId, requested_room_id: RoomId) -> Result<bool> {
        let mut rooms =
            world
                .select_mut(|entity| {
                    if let Ok(info) = entity.get::<RoomInfo>() {
                        if info.room_uid == requested_room_id {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
    
        if rooms.len() == 0 { // No room.
            let entity =
                world
                    .get(entity_id)?;
            let pkt = packet::error(packet::ErrorKind::NoRoom);
            entity::session::send(entity, pkt);
            return Ok(false);
        }
    
        let room = rooms.pop().unwrap();
    
        match entity::room::try_accept_client(room, *entity_id) {
            TryAcceptClientResult::InGame => {
                let entity =
                    world
                        .get(entity_id)?;
                // TODO: Find a packet to show that it's already playing.
                let pkt = packet::error(packet::ErrorKind::NoRoom);
                entity::session::send(entity, pkt);
                return Ok(false);
            },
            TryAcceptClientResult::FullOfClients => {
                let entity =
                    world
                        .get(entity_id)?;
                let pkt = packet::error(packet::ErrorKind::FullOfClientsInRoom);
                entity::session::send(entity, pkt);
                return Ok(false);
            },
            _ => {}
        }
    
        // Add OnRoomInfo to the user.
        let on_room_info = component::OnRoomInfo::new(room.id());
        let entity = world.get_mut(entity_id).unwrap();
        entity.push(on_room_info);
    
        // Change the ClientState.
        entity.push(ClientState::OnRoom);
    
        Ok(true)
    }
    
    let world = &mut server.world;
    let requested_room_id = pr.u8();

    if !try_enter_room(world, entity_id, requested_room_id)? {
        return Ok(());
    }

    let entity = world.get(entity_id).unwrap();
    let room_entity_id = entity.get::<OnRoomInfo>().unwrap().room_entity_id;

    // Send a packet to make the client enter the room.
    let pkt = {
        let room_entity = world.get(&room_entity_id).unwrap();
        let room_info = room_entity.get::<RoomInfo>().unwrap();
        let mut pw = PacketWriter::new(PACKET_ENTER_ROOM + 1);
        pw
            .u8(room_info.master_index as u8) // 8 (room owner index)
            .u8(room_info.map);   // 9 (map)
        // 10
        for i in 0..8 {
            if let Some(member_entity_id) = room_info.members[i] {
                let member_entity = world.get(&member_entity_id).unwrap();
                let member_info = member_entity.get::<AfterLoginInfo>().unwrap();
                let name = &member_info.user_schema.name;
                let client_uid = member_info.client_uid;
                let level = member_info.user_schema.level;
                let on_room_info = member_entity.get::<OnRoomInfo>().unwrap();
                let character = on_room_info.character;
                let state_in_room = on_room_info.state_in_room.clone();
                let team = on_room_info.team;
                pw
                    .u8(1) // an user exists.
                    .u8(character)   // (character)
                    .u8(state_in_room as u8) // (status, 1: not ready, 2: shopping, 3: ready)
                    .u8(level)  // (level)
                    .string(&name, 13)
                    .u8(team)    // 17 (team)
                    .u16(client_uid); // 18 (user uid?)
            } else {
                pw
                    .u8(0) // does not exists.
                    .pad(19);
            }
        }
        pw.as_vec()
    };
    entity::session::send(entity, pkt);

    // Notify other members that a newcomer joined.
    let room = world.get(&room_entity_id).unwrap();
    let pkt = {
        let room_entity = world.get(&room_entity_id).unwrap();
        let room_info = room_entity.get::<RoomInfo>().unwrap();
        let member_entity = world.get(&entity_id).unwrap();
        let member_info = member_entity.get::<AfterLoginInfo>().unwrap();
        let name = &member_info.user_schema.name;
        let client_uid = member_info.client_uid;
        let level = member_info.user_schema.level;
        let on_room_info = member_entity.get::<OnRoomInfo>().unwrap();
        let team = on_room_info.team;
        let index_in_room =
            room_info
                .members
                .iter()
                .enumerate()
                .find(|(_, member)| {
                    if let Some(member_entity_id) = member {
                        *member_entity_id == *entity_id
                    } else {
                        false
                    }
                })
                .unwrap()
                .0;
        let mut pw = PacketWriter::new(PACKET_ENTER_ROOM + 2);
		pw
            // 8
            .u8(index_in_room as u8) // the index of an user who has come newly.
		    // 13
		    .pad_to(13)
		    .u8(level) // level
		    .string(&name, 13)
		    // 27
		    .u8(team) // team
		    .u16(client_uid);
        pw.as_vec()
    };

    for member_id in entity::room::get_user_entity_ids(room) {
        let member_entity = world.get(&member_id).unwrap();
        entity::session::send(member_entity, pkt.clone());
    }
    
    Ok(())
}

pub fn handle(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    match pr.opcode() {
        PACKET_CHAT => {
            on_chat(server, entity_id, pr)
        },
        PACKET_ENTER_SHOP => {
            enter_shop(server, entity_id, pr)
        },
        PACKET_GO_BACK => {
            go_back(server, entity_id, pr)
        },
        PACKET_REQUEST_USER_LIST => {
            send_user_list(server, entity_id, pr)
        },
        PACKET_REQUEST_ROOM_LIST => {
            send_room_list(server, entity_id, pr)
        },
        PACKET_CREATE_ROOM => {
            create_room(server, entity_id, pr)
        },
        PACKET_ENTER_ROOM => {
            enter_room(server, entity_id, pr)
        },
        opcode => {
            println!("Unknown packet, OnLobby: {opcode}");
            Ok(())
        }
    }
}