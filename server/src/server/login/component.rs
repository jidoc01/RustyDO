// Copyright 2022 JungHyun Kim
// This file is part of RustyDO.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

//use legion::Entity;

use crate::prelude::*;

pub type RoomId = u8;
pub type MapKind = u8;

pub const MAX_CHAR_ID: u8 = 8;
pub const MAX_MAP_ID: u8 = 10;

pub const CHAR_RANDOM_ID: u8 = MAX_CHAR_ID;
pub const MAP_RANDOM_ID: u8 = MAX_MAP_ID;

pub const MAX_ROOMS: usize = 200;
pub const MIN_ROOM_ID: RoomId = 0;
pub const MAX_ROOM_ID: RoomId = (MAX_ROOMS - 1) as RoomId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClientState {
    BeforeLogin,
    AfterLogin,
    OnLobby,
    OnRoom,
    OnGame,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoomState {
    Waiting = 1,
    Full = 2,
    Playing = 3, // Playing = not (1 or 2)
}

#[derive(Clone, Debug, PartialEq)]
pub struct AfterLoginInfo {
    pub client_uid: ClientId,
    pub user_schema: UserSchema,
}

impl AfterLoginInfo {
    pub fn new(
            client_uid: ClientId,
            user_schema: UserSchema
        ) -> Self {
        Self {
            client_uid: client_uid,
            user_schema: user_schema,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StateInRoom {
    NotReady = 1,
    Shopping = 2,
    Ready = 3
}

#[derive(Clone, Debug, PartialEq)]
pub struct OnRoomInfo {
    pub room_entity_id: EntityId,
    pub team: u8,
    pub character: u8,
    pub state_in_room: StateInRoom,
}

impl OnRoomInfo {
    pub fn new(room_entity_id: EntityId) -> Self {
        Self {
            room_entity_id: room_entity_id,
            team: 0,
            character: CHAR_RANDOM_ID, // TODO: Remember his recent played character.
            state_in_room: StateInRoom::NotReady
        }
    }
}

// For checking ping-pong.
#[derive(Clone, Debug, PartialEq)]
pub struct PongInfo(pub bool);

#[derive(Clone, Debug, PartialEq)]
pub struct RoomInfo {
    pub room_uid: RoomId,
    pub master_index: usize,
    pub members: [Option<EntityId>; 8],
    pub is_playing: bool,

    pub name: String,
    pub password: String,
    pub map: MapKind,
    pub allows_item: bool,
    pub allows_evol: bool,
    pub max_clients: u8,
    pub level_condition: u8,
    pub level_base: u8,
}

impl RoomInfo {
    pub fn new(
            room_uid: RoomId,
            name: &str,
            password: &str,
            master_entity_id: EntityId,
            allows_item: bool,
            allows_evol: bool,
            max_clients: u8,
            level_condition: u8,
            level_base: u8
        ) -> Self {
        let mut members = [None; 8];
        members[0] = Some(master_entity_id);
        Self {
            room_uid: room_uid,
            is_playing: false,
            name: name.to_string(),
            password: password.to_string(),
            master_index: 0,
            map: MAP_RANDOM_ID,
            members: members,
            allows_item: allows_item,
            allows_evol: allows_evol,
            max_clients: max_clients,
            level_condition: level_condition,
            level_base: level_base
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerInfo {
    pub delay: u16,
    pub hp: u16,
    pub x: u16,
    pub y: u16,
}

impl PlayerInfo {
    pub fn default() -> Self {
        Self { 
            delay: 0,
            hp: 0,
            x: 0,
            y: 0
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
// Note that a Room entity can have this GameInfo.
pub struct GameInfo {
    pub room_eid: EntityId,

    /// It is used for checking either (1) game loading and (2) turn end.
    pub load_table: [bool; 8],

    pub turn_table: Vec<usize>,

    pub turn_index: usize,

    pub player_infos: [PlayerInfo; 8], // Represent states of in-game characters.
}
