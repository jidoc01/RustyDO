// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use std::cmp::Ordering;

use super::*;

pub fn create(world: &mut World, info: RoomInfo) -> &mut Entity {
    let id = world.push();
    let entity = world.get_mut(&id).unwrap();
    entity.push(EntityKind::Room);
    entity.push(info);
    entity
}

pub fn get_room_state(room: &Entity) -> Option<RoomState> {
    if let Ok(info) = room.get::<RoomInfo>() {
        if info.is_playing {
            Some(RoomState::Playing)
        } else {
            let count =
                info
                    .members
                    .iter()
                    .fold(0, |acc, &x| acc + if_else(x.is_some(), 1, 0));
            if count < info.max_clients {
                Some(RoomState::Waiting)
            } else {
                Some(RoomState::Full)
            }
        }
    } else {
        None
    }
}

pub enum TryAcceptClientResult {
    Ok,
    FullOfClients,
    InGame,
}

/// It tries to insert a client into a room.
pub fn try_accept_client(room: &mut Entity, entity_id: EntityId) -> TryAcceptClientResult {
    let state = get_room_state(room).unwrap();
    if state == RoomState::Full {
        return TryAcceptClientResult::FullOfClients;
    } else if state == RoomState::Playing {
        return TryAcceptClientResult::InGame;
    }

    let room_info = room
        .get_mut::<RoomInfo>()
        .unwrap();
    let empty_index = room_info
        .members
        .iter()
        .enumerate()
        .find(|v| v.1.is_none())
        .unwrap()
        .0;

    // Insert an user into a room.
    room_info.members[empty_index] = Some(entity_id);   
    
    TryAcceptClientResult::Ok
}

pub fn get_room_list(world: &World, is_waiting_only: bool, mode: u8, base_room_id: RoomId) -> Vec<&Entity> {
    let mut rooms = {
        let mut rooms = world
            .select(|entity| {
                if let Some(state) = get_room_state(entity) {
                    if is_waiting_only { // Waiting room only.
                        state == RoomState::Waiting
                    } else {
                        true
                    }
                } else {
                    false
                }
            });

        rooms.sort_by(|&room1, &room2| {
            if let (Ok(info1), Ok(info2)) = (room1.get::<RoomInfo>(), room2.get::<RoomInfo>()) {
                if_else(info1.room_uid < info2.room_uid, Ordering::Less, Ordering::Greater)
            } else {
                panic!("A room id should exist");
            }
        });

        rooms
    };

    match mode {
        1 => { // to the right
            let mut collected_rooms = vec!();
            for &room in rooms.iter() {
                let id = room.get::<RoomInfo>().unwrap().room_uid;
                if id >= base_room_id {
                    collected_rooms.push(room);

                    if collected_rooms.len() == 8 {
                        break;
                    }
                }
            }

            if collected_rooms.len() < 8 {
                for &room in rooms.iter().rev() {
                    let id = room.get::<RoomInfo>().unwrap().room_uid;
                    if id < base_room_id {
                        collected_rooms.insert(0, room);

                        if collected_rooms.len() == 8 {
                            break;
                        }
                    }
                }
            }

            rooms = collected_rooms;
        }, 
        2 => { // to the left
            let mut collected_rooms = vec!();
            for &room in rooms.iter().rev() {
                let id = room.get::<RoomInfo>().unwrap().room_uid;
                if id <= base_room_id {
                    collected_rooms.insert(0, room);

                    if collected_rooms.len() == 8 {
                        break;
                    }
                }
            }

            if collected_rooms.len() < 8 {
                for &room in rooms.iter() {
                    let id = room.get::<RoomInfo>().unwrap().room_uid;
                    if id > base_room_id {
                        collected_rooms.push(room);

                        if collected_rooms.len() == 8 {
                            break;
                        }
                    }
                }
            }
            
            rooms = collected_rooms;
        },
        _ => {
            if rooms.len() > 8 {
                rooms = rooms[..8].to_vec();
            }
        }
    }

    rooms
}

pub fn get_user_entity_ids(room: &Entity) -> Vec<EntityId> {
    let mut ret = vec!();
    let info= room.get::<RoomInfo>().unwrap();

    for place in info.members {
        if let Some(id) = place {
            ret.push(id);
        }
    }

    ret
}

pub fn get_index_in_room(room: &Entity, entity_id: &EntityId) -> Option<usize> {
    let info = room.get::<RoomInfo>().unwrap();

    for (i, place) in info.members.iter().enumerate() {
        if let Some(id) = place {
            if *id == *entity_id {
                return Some(i);
            }
        }
    }

    return None;
}

pub fn send_to_all(world: &World, room: &Entity, pkt: &Vec<u8>) {
    let info = room.get::<RoomInfo>().unwrap();

    for place in info.members.iter() {
        if let Some(id) = place {
            let entity = world
                .get(id)
                .unwrap();
            session::send(entity, pkt.clone());
        }
    }
}

pub fn send_to_all_except(world: &World, room: &Entity, pkt: &Vec<u8>, except_id: EntityId) {
    let info = room.get::<RoomInfo>().unwrap();

    for place in info.members.iter() {
        match place {
            Some(id) if *id != except_id => {
                let entity = world
                    .get(id)
                    .unwrap();
                session::send(entity, pkt.clone());
            },
            _ => {}
        }
    }
}