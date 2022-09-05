// RustyDO
//
// Copyright 2022. JungHyun Kim (jidoc01).
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{cmp::Ordering, rc::Rc};

use crate::prelude::*;
use super::*;

const PACKET_ON_FRAME: u8       = 32;
const PACKET_CHAT: u8           = 34;
const PACKET_END_LOADING: u8    = 83;
const PACKET_USE_ITEM: u8       = 85;
const PACKET_END_TURN: u8       = 86;
const PACKET_ON_DEATH: u8       = 88; // 88 - 128; On death.
const PACKET_GO_TO_LOBBY: u8    = 95;
const PACKET_EXIT_GAME: u8      = 128; // 128 - 95 - 57;

pub fn handle_exit(server: &mut Server, eid: &EntityId) {
    // Make the player die in the game.
    // TODO: It is not a safe way. We need to assure that this player is not
    // referenced from other spots in game logics.
    let _ = handle_on_death(server, eid);
}

fn on_chat(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    let unknown = pr.u8();
    let chat_kind = pr.u8(); // 0: team chat, 1: whisper, 2: all
    let text = pr.string(193);

    let name = world
        .get(entity_id)?
        .get::<AfterLoginInfo>()?
        .user_schema
        .name
        .clone();
    let room_eid = get_room_eid(world, entity_id);
    let room = world.get(&room_eid)?;
    let index_in_room = entity::room::get_index_in_room(room, entity_id).unwrap();

    // TODO: Emoticon.
    let pkt = packet::chat_in_game(
        unknown, chat_kind, &name, &text, 
        None, Some(index_in_room as u8)
    );
    
    let users = match chat_kind {
        0 => { // To all
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
                        if let Ok(member_entity) = world.get(member_entity_id) {
                            users.push(member_entity);
                        }
                    }
                });
            users
        },
        1 => { // Whisper
            // TODO
            vec!()
        },
        2 => { // Team chat
            let entity = world.get(entity_id)?;
            let on_room_info = entity.get::<OnRoomInfo>()?;
            let team_no = on_room_info.team;
            let room_entity_id = on_room_info.room_entity_id;
            let room_entity = world.get(&room_entity_id)?;
            let room_info = room_entity.get::<RoomInfo>()?;
            let mut users = vec!();
            room_info
                .members
                .iter()
                .for_each(|member| {
                    if let Some(member_entity_id) = member {
                        if let Ok(member_entity) = world.get(member_entity_id) {
                            let on_room_info = member_entity
                                .get::<OnRoomInfo>()
                                .unwrap();
                            let curr_team_no = on_room_info.team;
                            if team_no == curr_team_no {
                                users.push(member_entity);
                            }
                        }
                    }
                });
            users
        },
        _ => bail!("Invalid chat kind: {chat_kind}")
    };

    for user in users {
        entity::session::send(user, pkt.clone());
    }
        
    Ok(())
}

fn make_turn_change_pkt(
        index_in_room: u8,
        wind_velocity: i8,
        item: Option<(u8, u16)>,
        kurumon: Option<u16>,
        delay_list: &Vec<u16>,
        hp_list: &Vec<u32>
    ) -> Vec<u8> {
    let (item_no, item_pos) = item.unwrap_or((0, 0));

    let mut pw = PacketWriter::new(PACKET_END_TURN + 1);
    // 8
    pw
        .i32(wind_velocity as i32) // -30 ~ 30 (wind velocity)
    // 12 unknown
        .u8(1) // 1 or 0
        .u8(0) // if [12] == 0
    // 14~16 kurumon
        .u8(if_else(kurumon.is_some(), 0, 1)) // 1 or 0
        .pad(1)
        .u16(kurumon.unwrap_or(0)) // if [14] == 0 then kurumon position
    // 18 unknown
        .u8(0x00) // if [18] & 0x10 != 0 then [18] & 0x0F (low bits)
    // 19~20 item
        .u8(item_no) // item (0: no item)
        .u16(item_pos) // item position
    // 22
        .u8(1) // visible
    // 23
        .u8(index_in_room as u8); // whose turn
    // 24
    for i in 0..8 {
        pw
            .u16(0) // TODO: unknown
            .pad(2)
            .u32(hp_list[i]) // hp
            .pad(4);
    }
    // 120 unk
    pw
        .u16(0)
    // 122 unk
        .u16(0);
    pw.as_vec()
}

fn send_to_all(world: &World, room_eid: &EntityId, pkt: &Vec<u8>) {
    let room = world
        .get(&room_eid)
        .unwrap();
    entity::room::send_to_all(world, room, pkt);
}

fn send_to_all_except(world: &World, room_eid: &EntityId, pkt: &Vec<u8>, except_id: EntityId) {
    let room = world
        .get(&room_eid)
        .unwrap();
    entity::room::send_to_all_except(world, room, pkt, except_id);
}

fn get_index_in_room_in_turn(world: &World, room_eid: &EntityId) -> usize {
    let room = world
        .get(&room_eid)
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();
    let turn_index = game_info.turn_index;
    let index_in_room = game_info.turn_table[turn_index];
    index_in_room
}

fn get_room_eid(world: &World, eid: &EntityId) -> EntityId {
    let entity = world
        .get(eid)
        .unwrap();
    let on_room_info = entity
        .get::<OnRoomInfo>()
        .unwrap();
    on_room_info.room_entity_id
}

fn get_delay_list(world: &World, room_eid: &EntityId) -> Vec<u16> {
    let room = world    
        .get(room_eid)
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();

    (0..8)
        .map(|i| game_info.player_infos[i].delay as u16)
        .collect()
}

fn get_hp_list(world: &World, room_eid: &EntityId) -> Vec<u32> {
    let room = world    
        .get(room_eid)
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();

    (0..8)
        .map(|i| game_info.player_infos[i].hp as u32)
        .collect()
}

const MAX_WIND_VELOCITY: i8 = 30;

fn change_turn(world: &mut World, room_eid: &EntityId, is_starting_game: bool) {
    // Move to the next turn.
    {
        let room = world
            .get_mut(&room_eid)
            .unwrap();
        if !is_starting_game {
            // Try to find the next player who has not played yet and is still
            // alive.
            let game_info = room
                .get_mut::<GameInfo>()
                .unwrap();
            let turn_table_len = game_info.turn_table.len();
            let mut idx = game_info.turn_index + 1;
            while idx < turn_table_len {
                let i = game_info.turn_table[idx];
                let hp = game_info.player_infos[i].hp;
                if hp != 0 {
                    break;
                }
                idx += 1;
            }
            game_info.turn_index = idx;
        }
    }

    // Check if a cycle is finished.
    let room = world
        .get(room_eid)
        .unwrap();
    let room_info = room
        .get::<RoomInfo>()
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();
    let turn_index = game_info.turn_index;
    let turn_table_len = game_info.turn_table.len();
    if turn_index >= turn_table_len { // it has finished a cycle.
        // Set a new cycle.
        // The order is decided by delays of the previous cycle.
        let mut players_with_delay = vec!();
        for i in 0..8 {
            if room_info
                .members[i]
                .is_some() {
                let delay = game_info.player_infos[i].delay;
                let hp = game_info.player_infos[i].hp;
                if hp != 0 { // Assure that his hp is not zero.
                    players_with_delay.push((delay, i));
                }
            }
        }

        // Reorder it.
        players_with_delay.sort_by(|(delay1, _), (delay2, _)| {
            if *delay1 < *delay2 {
                Ordering::Less
            } else if *delay1 > *delay2 {
                Ordering::Greater
            } else {
                // Grant its priority randomly.
                match rand::random::<u8>() % 3 {
                    0 => Ordering::Less,
                    1 => Ordering::Equal,
                    2 => Ordering::Greater,
                    _ => panic!("Modulo 3 can not be more than 2")
                }
            }
        });

        {
            let room = world
                .get_mut(room_eid)
                .unwrap();
            let game_info = room
                .get_mut::<GameInfo>()
                .unwrap();

            game_info.turn_table = players_with_delay
                .iter()
                .map(|(_, idx)| *idx)
                .collect();

            game_info.turn_index = 0;
        }
    }

    // Unset load table.
    {
        let room = world
            .get_mut(room_eid)
            .unwrap();
        let game_info = room
            .get_mut::<GameInfo>()
            .unwrap();

        for i in 0..8 {
            game_info.load_table[i] = false;
        }
    }

    // send to all that a turn has been changed.
    {
        let index_in_room = get_index_in_room_in_turn(world, room_eid);
        let delay_list = get_delay_list(world, room_eid);
        let hp_list = get_hp_list(world, room_eid);
        let wind_velocity = rand::random::<i8>() % (MAX_WIND_VELOCITY + 1); // -30 ~ 30
        let pkt = make_turn_change_pkt(index_in_room as u8, wind_velocity, None, None, &delay_list, &hp_list);
        send_to_all(world, room_eid, &pkt);
    }
}

fn check_if_his_turn(world: &World, eid: &EntityId) -> bool {
    let room_eid = get_room_eid(world, eid);
    let room = world
        .get(&room_eid)
        .unwrap();
    let index_in_room = entity::room::get_index_in_room(room, eid).unwrap();
    let index_in_room_in_turn = get_index_in_room_in_turn(world, &room_eid);
    index_in_room == index_in_room_in_turn
}

fn check_if_load_finished(world: &World, room_eid: &EntityId) -> bool {
    let room = world
        .get(&room_eid)
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();
    let room_info = room
        .get::<RoomInfo>()
        .unwrap();

    (0..8)
        .all(|i| {
            let user_exists = room_info
                .members[i]
                .is_some();
            let is_loaded = game_info.load_table[i];
            !user_exists || is_loaded
        })
}

/// Return Some(team) where team is an id of the team which has won the game.
fn check_if_game_finished(world: &World, room_eid: &EntityId) -> Option<usize> {
    let mut survivors_by_team = [0usize; 8];
    let room = world
        .get(room_eid)
        .unwrap();
    let game_info = room
        .get::<GameInfo>()
        .unwrap();
    let room_info = room
        .get::<RoomInfo>()
        .unwrap();

    for i in 0..8 {
        if let Some(member_eid) = room_info.members[i] {
            let member = world
                .get(&member_eid)
                .unwrap();
            let on_room_info = member
                .get::<OnRoomInfo>()
                .unwrap();
            let team_id = on_room_info.team;
            let hp = game_info.player_infos[i].hp;
            if hp > 0 {
                survivors_by_team[team_id as usize] += 1;
            }
        }
    }
    
    let survived_teams = survivors_by_team
        .iter()
        .filter(|&&n| n > 0)
        .count();
    // TODO: Needs death-match.
    if survived_teams == 0 {
        todo!("Death match");
    } else if survived_teams == 1 {
        let (team_id, _n) = survivors_by_team
            .iter()
            .enumerate()
            .find(|(_, &n)| n > 0)
            .unwrap();
        Some(team_id)
    } else {
        None
    }
}

fn set_client_states_to_on_room(world: &mut World, room_eid: &EntityId) {
    let ids = {
        let room =
            world
                .get(&room_eid)
                .unwrap();
        entity::room::get_user_entity_ids(room)
    };
    ids
        .iter()
        .for_each(|id| {
            world
                .get_mut(&id)
                .unwrap()
                .push(ClientState::OnRoom);
        });
}

fn end_game(world: &mut World, room_eid: &EntityId, winner_team: usize) {
    // Send packets to end this game.
    let pkt = PacketWriter::new(90)
        .u8(winner_team as u8)
        .u8(0) // TODO: unknown (boolean)
        .as_vec();
    send_to_all(world, &room_eid, &pkt);

    // Change clients' states to OnRoom.
    set_client_states_to_on_room(world, room_eid);

    {
        let room = world
            .get_mut(room_eid)
            .unwrap();
        let room_info = room
            .get_mut::<RoomInfo>()
            .unwrap();

        // Unset is_playing.
        room_info.is_playing = false;

        // Unset players' readiness.
        let ids = {
            let room = world
                .get(&room_eid)
                .unwrap();
            entity::room::get_user_entity_ids(room)
        };
        ids
            .iter()
            .for_each(|id| {
                world
                    .get_mut(&id)
                    .unwrap()
                    .get_mut::<OnRoomInfo>()
                    .unwrap()
                    .state_in_room = StateInRoom::NotReady;
            });
    }
}

pub fn loop_main(world: &mut World, room_eid: EntityId) {
    // Start a turn.
    change_turn(world, &room_eid, true);

    let turn_duration = Duration::from_secs(60 * 2);
    let mut turn_instant = Instant::now();
    register_loop!(world, move |server: &mut Server, _timer_eid: &EntityId| -> bool {
        let world = &mut server.world;

        let is_turn_finished = check_if_load_finished(world, &room_eid);
        let is_turn_timeout = turn_instant.elapsed().ge(&turn_duration);
        if is_turn_finished || is_turn_timeout { // Go to the next turn.
            if is_turn_timeout {
                // TODO: Check who has timed out and kick him off.
                todo!("kick the player who caused turn timeout");
            }

            // Check if the game has ended.
            if let Some(team) = check_if_game_finished(world, &room_eid) {
                end_game(world, &room_eid, team);
                return false;
            }

            // Start a new turn.
            change_turn(&mut server.world, &room_eid, false);

            // Reset the turn timer.
            turn_instant = Instant::now();
        }

        return true;
    });
}

pub fn loop_loading(world: &mut World, room_eid: EntityId) {
    let duration = Duration::from_secs(30); // At most 30 secs.
    let instant = Instant::now();

    register_loop!(world, move |server: &mut Server, _timer_eid: &EntityId| -> bool {
        let world = &mut server.world;
        // Check if all ready, or it is timeout.
        let is_all_loaded = check_if_load_finished(world, &room_eid);
        let is_timeout = instant.elapsed().ge(&duration);
        if is_timeout {
            // TODO: Players who are not yet ready for battle must be kicked off.
            // It might be due to that (1) he left the server (2) or it is on a low latency.
            todo!("Handle players with too long loading time");
        }
        if is_all_loaded || is_timeout {
            loop_main(world, room_eid);

            return false;
        }

        return true;
    });
}

fn on_frame(server: &mut Server, eid: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;

    if !check_if_his_turn(world, eid) {
        return Ok(()); // TODO: Hack?
    }

	pr.seek(4);
    let left = pr.left();
    let movement_data = pr.vec(left);

    // Send this information to the others (except himself).
    let pkt = {
        let mut pw = PacketWriter::new(PACKET_ON_FRAME + 1);
        pw.pad_to(27);
        pw.vec(&movement_data);
        pw.as_vec()    
    };

    let room_eid = get_room_eid(world, eid);
    send_to_all_except(world, &room_eid, &pkt, *eid);
    Ok(())
}

fn end_loading(server: &mut Server, eid: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &server.world;
    
    // character state values.
    // 0: delay
    // 1: current hp
    // 2: x (right)
    // 3: y (down)
    let words: Vec<u16> = (0..4).map(|_| pr.u16()).collect();
    
    let room_eid = get_room_eid(world, eid);
    let room = server
        .world
        .get_mut(&room_eid)
        .unwrap();
    let index_in_room = entity::room::get_index_in_room(room, eid).unwrap();
    let game_info = room
        .get_mut::<GameInfo>()
        .unwrap();

    game_info.load_table[index_in_room] = true;

    game_info.player_infos[index_in_room].delay = words[0];
    game_info.player_infos[index_in_room].hp    = words[1];
    game_info.player_infos[index_in_room].x     = words[2];
    game_info.player_infos[index_in_room].y     = words[3];

    Ok(())
}

fn use_item(server: &mut Server, eid: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &server.world;
    
    let left = pr.left();
    let bytes = pr.vec(left);

    let pkt = PacketWriter::new(PACKET_USE_ITEM)
        .vec(&bytes)
        .as_vec();
    let room_eid = get_room_eid(world, eid);
    send_to_all_except(world, &room_eid, &pkt, *eid);
    Ok(())
}

fn end_turn(server: &mut Server, eid: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    
    // character state values.
    let words: Vec<u16> = (0..4).map(|_| pr.u16()).collect();

    let room_eid = get_room_eid(world, eid);
    let room = world
        .get_mut(&room_eid)
        .unwrap();

    let index_in_room = entity::room::get_index_in_room(room, eid).unwrap();
    let game_info = room
        .get_mut::<GameInfo>()
        .unwrap();

    game_info.load_table[index_in_room] = true;

    game_info.player_infos[index_in_room].delay = words[0];
    game_info.player_infos[index_in_room].hp    = words[1];
    game_info.player_infos[index_in_room].x     = words[2];
    game_info.player_infos[index_in_room].y     = words[3];

    Ok(())
}

fn make_on_death_packet(index_in_room: usize) -> Vec<u8> {
    PacketWriter::new(PACKET_ON_DEATH + 1)
        .u8(index_in_room as u8)
        .as_vec()
}

fn handle_on_death(server: &mut Server, eid: &EntityId) -> Result<()> {
    let world = &server.world;
    let room_eid = get_room_eid(world, eid);
    let pkt = {
        let room = world.get(&room_eid)?;
        let index_in_room = entity::room::get_index_in_room(room, eid).unwrap();
        make_on_death_packet(index_in_room)
    };
    send_to_all(world, &room_eid, &pkt);
    Ok(())
}

fn on_death(server: &mut Server, eid: &EntityId, _pr: PacketReader) -> Result<()> {
    handle_on_death(server, eid)
}

pub fn handle(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    match pr.opcode() {
        PACKET_CHAT => {
            // Re-use it.
            on_chat(server, entity_id, pr)
        },
        PACKET_ON_FRAME => {
            on_frame(server, entity_id, pr)
        },
        PACKET_END_LOADING => {
            end_loading(server, entity_id, pr)
        },
        PACKET_USE_ITEM => {
            use_item(server, entity_id, pr)
        },
        PACKET_END_TURN => {
            end_turn(server, entity_id, pr)
        },
        PACKET_ON_DEATH => {
            on_death(server, entity_id, pr)
        },
        opcode => {
            println!("Unknown packet, OnGame: {opcode}");
            Ok(())
        }
    }
}