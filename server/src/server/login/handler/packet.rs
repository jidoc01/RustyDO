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

use crate::{prelude::*};
use super::*;

pub enum ErrorKind {
    FullOfClients = 5001,
    InvalidId = 7001,
    NoResponse = 7003,
    CantFindUserInfo = 7005,
    Banned = 7007,
    Online = 7010,
    FullOfClientsInRoom = 7011,
    FullOfRooms = 7012,
    NoRoom = 7013,
    NotEnoughLevel = 7014,
    UnbalancedTeamNumber = 7016,
    BoardNotReady = 7021,
    SameNickname = 7027,
    NotEnoughTicketToChangeNickname = 7028,
    DupNickname = 7029,
    InvalidAccInfo = 8003,
}

pub fn error(err_kind: ErrorKind) -> Vec<u8> {
    PacketWriter::new(0)
        .u32(err_kind as u32)
        .as_vec()
}

pub fn chat(is_admin: bool, is_whisper: bool, name: &str, text: &str) -> Vec<u8> {
    PacketWriter::new(34)
        .u8(if_else(is_admin, 1, 0))
        .u8(if_else(is_whisper, 1, 0))
        .string(text, 222)
        .string(name, 13)
        .as_vec()
}

pub fn chat_in_game(
    unknown: u8,
    msg_kind: u8,
    name: &str,
    text: &str,
    emoticon: Option<u32>, // for in-game
    index_in_room: Option<u8> // for in-game
) -> Vec<u8> {
    PacketWriter::new(34)
        .u8(unknown)
        .u8(msg_kind) // type (1: whisper, 2: team?, 0: all?)
        .string(text, 206)
        .u32(emoticon.unwrap_or(0xffffffff)) // TODO: 0xff..ff: for in-game chat? / else: emoticon
        .pad(12)  
        .string(name, 13)
        .pad(31)
        // 276
        .u8(index_in_room.unwrap_or_default())
        .as_vec()
}

pub fn room_list(world: &World, rooms: &Vec<&Entity>) -> Vec<u8> {
    let mut pw = PacketWriter::new(58);
    pw.u8(rooms.len() as u8);
    for i in 0..8 {
        if i >= rooms.len() {
            pw.pad(46 + 14 * 8); // 8: max players
            continue;
        }
        let room = rooms[i];
        let state = entity::room::get_room_state(room).unwrap();
        let info = room.get::<RoomInfo>().unwrap();
        pw
		    .u8(state as u8) // 1: waiting, 2: full, else: playing
		    .string(&info.name, 24+1)
		    .string(&info.password, 12+1)
		    .u8(info.max_clients)       // 39 - 1 2 4 8 - max players
		    .u8(info.level_condition) // 40 - (1: same, 2: upper, 3: lower)
		    .u8(info.level_base)      // 41 - base level
		    .u8(if_else(info.allows_item, 1 << 0, 0) | if_else(info.allows_evol, 1 << 1, 0))
		    .u8(info.map) // 43 - map (0~)
		    .u8(info.room_uid) // 44 - room uid
            .u8(0);             // unknown
		// 46 - players in a room
        for member in info.members {
            if let Some(k) = member {
                if let Ok(entity) = world.get(&k) {
                    let user_info = entity.get::<AfterLoginInfo>().unwrap();
                    pw
                        .string(&user_info.user_schema.name, 13)
                        .u8(user_info.user_schema.level);
                } else {
                    pw.pad(14);
                }
            } else {
                pw.pad(14);
            }
        }
	};
    pw.as_vec()
}

/// It's used to notify the room members when someone exited.
pub fn room_notify_leaver_and_master(leaver_index: u8, master_index: u8) -> Vec<u8> {
    PacketWriter::new(96)
        .u8(leaver_index)
        .u8(master_index)
        .as_vec()
}

/// 
pub fn room_leave_to_lobby() -> Vec<u8> {
    // When a leaver index is -1 (0xff), then the client exits into the lobby.
    room_notify_leaver_and_master(u8::MAX, 0)
}