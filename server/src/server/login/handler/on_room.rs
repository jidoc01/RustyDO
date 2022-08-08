// Copyright 2022 JungHyun Kim
// This file is part of Foobar.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use rand::prelude::SliceRandom;

use crate::prelude::*;
use super::*;

const PACKET_CHAT: u8           = 34;
const PACKET_ENTER_SHOP: u8     = 50;
const PACKET_LEAVE_SHOP: u8     = 52;
const PACKET_CHANGE_MAP: u8     = 66;
const PACKET_CHANGE_CHAR: u8    = 69;
const PACKET_CHANGE_TEAM: u8    = 71;
const PACKET_GET_READY: u8      = 76;
const PACKET_GET_NOT_READY: u8  = 78;
const PACKET_START_GAME: u8     = 80;
const PACKET_EXIT_ROOM: u8      = 95;
const PACKET_KICK: u8           = 98;

fn enter_shop(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    // Change his state.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.state_in_room = StateInRoom::Shopping;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_ENTER_SHOP + 1)
            .u8(index_in_room as u8)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn leave_shop(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    // Change his state.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.state_in_room = StateInRoom::NotReady;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_LEAVE_SHOP + 1)
            .u8(index_in_room as u8)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn change_map(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let map_id = pr.u8();

    let world = &mut server.world;
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get_mut(&room_entity_id).unwrap();
    let room_info = room_entity.get_mut::<RoomInfo>().unwrap();

    // Check if it is an owner of a room.
    if room_info.members[room_info.master_index].unwrap() != *entity_id {
        bail!("It tried to change a map although it is an an owner of a room");
    }

    // Change a map id of the room.
    room_info.map = map_id;

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_CHANGE_MAP + 1)
            .u8(map_id)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn change_char(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let char_id = pr.u8();

    let world = &mut server.world;
    // Change the character id.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.character = char_id;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_CHANGE_CHAR + 1)
            .u8(index_in_room as u8)
            .u8(char_id)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn change_team(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let team_id = pr.u8();

    let world = &mut server.world;
    // Change the team.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.team = team_id;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_CHANGE_TEAM + 1)
            .u8(index_in_room as u8)
            .u8(team_id)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn get_ready(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    // Change his readiness.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.state_in_room = StateInRoom::Ready;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_GET_READY + 1)
            .u8(index_in_room as u8)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn get_not_ready(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    // Change his readiness.
    {
        let entity = world.get_mut(entity_id).unwrap();
        let on_room_info = entity.get_mut::<OnRoomInfo>().unwrap();
        on_room_info.state_in_room = StateInRoom::NotReady;
    }
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let index_in_room = entity::room::get_index_in_room(room_entity, entity_id).unwrap();

    // Send packets to notify that it has changed its map.
    let pkt =
        PacketWriter::new(PACKET_GET_NOT_READY + 1)
            .u8(index_in_room as u8)
            .as_vec();
    for member in room_info.members {
        if let Some(member_entity_id) = member {
            let member = world.get(&member_entity_id).unwrap();
            entity::session::send(member, pkt.clone());
        }
    }

    Ok(())
}

fn start_game(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    let room_eid = {
        let entity =
            world
                .get(entity_id)?;
        let on_room_info =
            entity
                .get::<OnRoomInfo>()?;
        on_room_info.room_entity_id
    };

    {
        let room_entity_id = room_eid;
        let room_info =
            world
                .get(&room_entity_id)?
                .get::<RoomInfo>()?;
        let map_id = match room_info.map {
            MAP_RANDOM_ID => rand::random::<u8>() % MAX_MAP_ID,
            id => id
        };
        let initial_positions = {
            let mut rng = rand::thread_rng();
            let mut positions: Vec<usize> = (0..16).collect();
            positions.shuffle(&mut rng);
            positions[0..8].to_vec()     
        };
        
        // TODO
        let mut pw = PacketWriter::new(PACKET_START_GAME + 1);
        pw
            .u8(map_id)
            .pad_to(12);
        for i in 0..8 {
            if let Some(member_entity_id) = room_info.members[i] {
                let member =
                    world
                        .get(&member_entity_id)?;
                let on_room_info =
                    member
                        .get::<OnRoomInfo>()?;
                let char_id = match on_room_info.character {
                    CHAR_RANDOM_ID => rand::random::<u8>() % MAX_CHAR_ID,
                    id => id
                };
                let initial_pos = initial_positions[i];
                pw
                    .u8(0) // TODO: unknown.
                    .u8(initial_pos as u8) // starting position ?
                    .u8(char_id) // character id
                    .u8(0) // TODO: Unknown, maybe dummy.
                    .u32(0); // TODO: unknown.
            } else {
                pw.pad(8);
            }
        }

        let pkt = pw.as_vec();

        {
            let room =
                world
                    .get(&room_entity_id)?;
            entity::room::send_to_all(world, room, &pkt);
        }
    }

    {
        let room =
            world
                .get(&room_eid)?;
        let room_info =
            room
                .get::<RoomInfo>()?;

        let mut effective_indices = vec!();
        for i in 0..8 {
            if room_info
                .members[i]
                .is_some() {
                effective_indices.push(i);
            }
        }

        // Add a GameInfo to the Room entity.
        let turn_table = { // get_turn_table();
            // The order is randomized since it is the first cycle.
            let mut members_by_team: Vec<Vec<usize>> = (0..8)
                .map(|_| vec!())
                .collect();
            // 1. By-Team order.
            // 2. In-team order.
            for idx in effective_indices {
                let member_eid = room_info.members[idx].unwrap();
                let entity = world
                    .get(&member_eid)?;
                let team_id = entity
                    .get::<OnRoomInfo>()?
                    .team;
                members_by_team[team_id as usize].push(idx);
            }
            
            let mut rng = rand::thread_rng();
            // Shuffle by team.
            members_by_team.shuffle(&mut rng);
            // Shuffle by player in each team, and ignore an empty team.
            for players in members_by_team.iter_mut() {
                if !players.is_empty() {
                    players.shuffle(&mut rng);
                }
            }

            let mut turn_table = vec!();
            
            loop {
                let mut has_pushed = false;
                for team_id in 0..8 {
                    let members = &mut members_by_team[team_id];
                    if let Some(idx_in_room) = members.pop() {
                        turn_table.push(idx_in_room);
                        has_pushed = true;
                    }
                }

                if !has_pushed {
                    break;
                }
            }
            
            turn_table
        };

        let game_info = GameInfo {
            room_eid: room_eid,
            load_table: [false; 8],
            turn_table: turn_table,
            turn_index: 0,
            player_infos: [PlayerInfo::default(); 8]
        };
        
        let room =
            world
                .get_mut(&room_eid)?;

        room.push(game_info);
        
        let room_info =
            room
                .get_mut::<RoomInfo>()?;
        
        // Set room's state to playing.
        room_info.is_playing = true;
    }

    // Set all players' state to OnGame.
    {
        let ids = {
            let room = world
                .get(&room_eid)?;
            entity::room::get_user_entity_ids(room)
        };
        ids
            .iter()
            .for_each(|id| {
                world
                    .get_mut(&id)
                    .unwrap()
                    .push(ClientState::OnGame);
            });
    }

    // Start a loop for loading resources in clients.
    on_game::loop_loading(world, room_eid);

    log!("[{}] Start a game", room_eid);

    Ok(())
}

/// This can be called generally for handling exit.
/// Note that it can be called even when it's playing a game.
pub fn handle_exit(server: &mut Server, entity_id: &EntityId) {
    let room_entity_id = server
        .world
        .get(entity_id)
        .unwrap()
        .get::<OnRoomInfo>()
        .unwrap()
        .room_entity_id;

    // Check if it is in-game.
    {
        let is_playing = server
            .world
            .get(&room_entity_id)
            .unwrap()
            .get::<RoomInfo>()
            .unwrap()
            .is_playing;

        if is_playing {
            on_game::handle_exit(server, entity_id);
        }
    }

    {
        let world = &mut server.world;
        let room_entity = world
            .get_mut(&room_entity_id)
            .unwrap();

        let found_index = {
            let room_info = room_entity.get_mut::<RoomInfo>().unwrap();

            let found_index = {
                let mut found_index = None;
                for (i, place) in room_info.members.iter_mut().enumerate() {
                    if let Some(user_entity_id) = place {
                        if *user_entity_id == *entity_id {
                            found_index = Some(i);
                            break;
                        }
                    }
                }
                found_index
            };
            
            if found_index.is_none() {
                panic!("A place can not be empty");
            }

            let found_index = found_index.unwrap();
            room_info.members[found_index] = None;

            found_index
        };

        // (1) Check if it is an empty room now.
        if entity::room::get_user_entity_ids(room_entity).is_empty() {
            // Remove the room.
            world.remove(&room_entity_id);
        } else {
            // (2) Check if he was the owner of the room.
            let room_info = room_entity.get_mut::<RoomInfo>().unwrap();
            let mut master_index = room_info.master_index;
            if master_index == found_index {
                // If so, then give his ownership to the other member.
                for (i, place) in room_info.members.iter().enumerate() {
                    if place.is_some() {
                        master_index = i;
                    }
                }
                room_info.master_index = master_index;
            }

            // (3) Notify the other members that he has left the room.
            let leaver = found_index as u8;
            let master = master_index as u8; 
            let pkt = packet::room_notify_leaver_and_master(leaver, master);
            for entity_id in entity::room::get_user_entity_ids(room_entity) {
                let member = world.get(&entity_id).unwrap();
                entity::session::send(member, pkt.clone());
            }
        }
    }

    // Change entity's state.
    {
        let entity = server
            .world
            .get_mut(entity_id)
            .unwrap();
        entity.push(ClientState::OnLobby);
        entity.remove::<OnRoomInfo>(); // Remove OnRoomInfo.
    }
}

fn kick(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let index_in_room_to_kick = pr.u8() as usize;

    let world = &mut server.world;
    let entity = world.get(entity_id).unwrap();
    let on_room_info = entity.get::<OnRoomInfo>().unwrap();
    let room_entity_id = on_room_info.room_entity_id;
    let room_entity = world.get(&room_entity_id).unwrap();
    let room_info = room_entity.get::<RoomInfo>().unwrap();
    let entity_id_to_kick = room_info.members[index_in_room_to_kick].unwrap();

    // Make the client leave the room.
    handle_exit(server, &entity_id_to_kick);

    // Send an exit packet.
    let world = &mut server.world;
    let entity = world.get_mut(&entity_id_to_kick).unwrap();
    let pkt = packet::room_leave_to_lobby();
    entity::session::send(entity, pkt);

    Ok(())
}

fn exit_room(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    handle_exit(server, entity_id);

    // Send an exit packet.
    let world = &mut server.world;
    let entity = world.get_mut(entity_id).unwrap();
    let pkt = packet::room_leave_to_lobby();
    entity::session::send(entity, pkt);

    Ok(())
}

pub fn handle(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    match pr.opcode() {
        PACKET_CHAT => {
            // Re-use it.
            on_lobby::on_chat(server, entity_id, pr)
        },
        PACKET_ENTER_SHOP => {
            enter_shop(server, entity_id, pr)
        },
        PACKET_LEAVE_SHOP => {
            leave_shop(server, entity_id, pr)
        },
        PACKET_CHANGE_MAP => {
            change_map(server, entity_id, pr)
        },
        PACKET_CHANGE_CHAR => {
            change_char(server, entity_id, pr)
        },
        PACKET_CHANGE_TEAM => {
            change_team(server, entity_id, pr)
        },
        PACKET_GET_READY => {
            get_ready(server, entity_id, pr)
        },
        PACKET_GET_NOT_READY => {
            get_not_ready(server, entity_id, pr)
        },
        PACKET_START_GAME => {
            start_game(server, entity_id, pr)
        },
        PACKET_EXIT_ROOM => {
            exit_room(server, entity_id, pr)
        },
        PACKET_KICK => {
            kick(server, entity_id, pr)
        },
        opcode => {
            println!("Unknown packet, OnRoom: {opcode}");
            Ok(())
        }
    }
}