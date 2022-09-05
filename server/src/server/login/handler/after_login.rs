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

use std::borrow::Borrow;

use rusqlite::{params, NO_PARAMS};

use crate::prelude::*;
use super::*;

const PACKET_FIND_NAME: u8          = 7;
const PACKET_REQUEST_NOTICE: u8     = 16;
const PACKET_REQUEST_TICKER: u8     = 20;
//
const PACKET_ENTER_BOARD: u8        = 26;
const PACKET_READ_ARTICLE: u8       = 28;
const PACKET_WRITE_ARTICLE: u8      = 30;
//
const PACKET_UPDATE_SETTINGS: u8    = 41;
const PACKET_REGISTER_NAME: u8      = 44;
const PACKET_ENTER_LOBBY: u8        = 57;
const PACKET_CHANGE_NICKNAME: u8    = 123;

fn get_notice_msg(server: &Server) -> String {
    server
        .config
        .message
        .notice
        .replace("\n", "\r\n\r\n") // Add carriage returns in front of the line breaks.
}

fn send_notice(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    let notice_msg = get_notice_msg(server);
    let pkt = PacketWriter::new(PACKET_REQUEST_NOTICE + 1)
        .string_with_null(&notice_msg)
        .as_vec();
    let entity = server.world.get(entity_id).unwrap();
    entity::session::send(entity, pkt);
    Ok(())
}

fn send_ticker(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let msg_initial = "접속하고 싶은 채널을 선택해라 동~글.";
    let msg_board = "읽고 싶은 글을 선택해라 동~글.";
    let msg_ranking = "테이머들의 랭킹을 볼 수 있다 동~글.";
    let msg_selection = "디지몬 온라인의 세계에 잘 왔다 동~글.";
    let pkt = PacketWriter::new(PACKET_REQUEST_TICKER + 1)
        .string(msg_initial, 101)
        .string(msg_board, 101)
        .string(msg_ranking, 101)
        .string(msg_selection, 101)
        .as_vec();
    entity::session::send_by_eid(&server.world, entity_id, pkt);
    Ok(())
}

const POST_MAX_VISILITY: usize = 11;

fn get_board_info_pkt(server: &Server, board_idx: usize) -> Vec<u8> {
    // TODO: Clean up.
    let max_post_id = {
        let db = &server.db;
        let query = format!("SELECT MAX(JSON_EXTRACT(data, '$.post_id')) from {POST_TBL}");
        let mut stmt = db
            .as_ref()
            .prepare(&query)
            .unwrap();
        let mut ret = stmt
            .query(NO_PARAMS)
            .unwrap();
        match ret
            .next()
            .unwrap() {
            Some(row) => match row.get::<_, u32>(0) {
                Ok(max_post_id) => max_post_id,
                _ => 1 // It can happen when its type is Null.
            }
            None => 1,
        }
    };
    // Note that post id starts from 1.
    // To allow negative numbers, we use isize.
    let post_id_ub = (max_post_id as isize) - ((board_idx * POST_MAX_VISILITY) as isize);
    let post_id_lb = post_id_ub - (POST_MAX_VISILITY as isize) + 1;
    let posts = {
        let db = &server.db;
        let posts: Vec<PostSchema> = db
            .table(POST_TBL)
            .unwrap()
            .iter()
            .filter(field("post_id").gte(post_id_lb).and(field("post_id").lte(post_id_ub)))
            .take(POST_MAX_VISILITY as u32)
            .data(db)
            .unwrap();
        posts
    };
    
    let mut pw = PacketWriter::new(PACKET_ENTER_BOARD + 1);

    pw
        .u8(posts.len() as u8) // maximum: 11 (will cause overflow if it exceeds 11)
        .pad(3);
    for post in posts.iter().rev() {
        let name = {
            let db = &server.db;
            db
                .table(USER_TBL)
                .unwrap()
                .iter()
                .filter(field("id").eq(&post.writer_id))
                .field::<String, _>("name", db)
                .unwrap()
                .first()
                .map_or("알 수 없음".into(), |name| name.to_owned())
        };
        let y = post.datetime.year as u32;
        let m = post.datetime.month;
        let d = post.datetime.day;
        let h = post.datetime.hour;
        let min = post.datetime.min;
        pw
            .u8(if_else(post.is_notice, 1, 0)) // 0: is_admin (server notice)
            .pad(3)
            .u32(post.post_id) // 4: article unique id
            .u32(post.post_id) // 8: article number (to be shown in the list)
            .string(if_else(post.is_deleted, "삭제된 글입니다.", &post.title), 48) // 12: title
            .pad(1) // 60: null terminator
            .string(&name, 12) // 61: writter
            .pad(76 - 73) // 73
            .u32((y << 16) | (m << 8) | (d << 0)) // 76: y(2)/m(1)/d(1) (in a word)
            .u32((h << 16) | (min << 8)) // 80: hour(2)/min(1)/dummy(1) (in a word)
            .u16(post.views) // 84: 조회수
            .u8(0) // 86: n-th reply (0: not a reply)
            .pad(1); // an unit is 88 bytes. 
    }

    pw.as_vec()
}

/// It is called when either (1) enter a board or (2) request the list of articles.
fn enter_board(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    //let pkt = packet::error(ErrorKind::BoardNotReady);
    let board_idx = pr.u32() as usize;
    let pkt = get_board_info_pkt(server, board_idx);
    entity::session::send_by_eid(&server.world, entity_id, pkt);
    Ok(())
}

const POST_MAX_TEXT_LEN: usize = 2048; 

fn get_post_text_from_post_id(db: &Connection, post_id: u32) -> Option<String> {
    db
        .table(POST_TBL)
        .unwrap()
        .iter()
        .filter(field("post_id").eq(post_id))
        .field::<String, _>("text", db)
        .unwrap()
        .first()
        .and_then(|text| Some(text.to_owned()))
}

fn read_article(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let post_id = pr.u32();

    let db = &server.db;

    let is_deleted = db
        .table(POST_TBL)?
        .iter()
        .filter(field("post_id").eq(post_id))
        .field::<bool, _>("is_deleted", db)?
        .first()
        .unwrap()
        .to_owned();
    if is_deleted {
        // TODO: Use adequate packet.
        let pkt = packet::error(ErrorKind::BoardNotReady);
        entity::session::send_by_eid(&server.world, entity_id, pkt);
        return Ok(());
    }

    let text = match get_post_text_from_post_id(db, post_id) {
        Some(text) => text,
        None => {
            // TODO: Use adequate packet.
            let pkt = packet::error(ErrorKind::BoardNotReady);
            entity::session::send_by_eid(&server.world, entity_id, pkt);
            return Ok(());
        }
    };
    // Increase its views.
    // TODO: Optimization with json_set query.
    {
        let views = db
            .table(POST_TBL)?
            .iter()
            .filter(field("post_id").eq(post_id))
            .field::<isize, _>("views", db)?
            .first()
            .map_or(0, |views| views.to_owned());
        db
            .table(POST_TBL)?
            .iter()
            .filter(field("post_id").eq(post_id))
            .set("views", views + 1, db)?;
    }

    let pkt = PacketWriter::new(PACKET_READ_ARTICLE + 1)
        .string(&text, POST_MAX_TEXT_LEN)
        .as_vec();
    let entity = server.world.get(entity_id).unwrap();
    entity::session::send(entity, pkt);
    Ok(())
}

fn assign_new_post_id(db: &Connection) -> u32 {
    let query = format!("SELECT MAX(JSON_EXTRACT(data, '$.post_id')) from {POST_TBL}");
    let mut stmt = db
        .as_ref()
        .prepare(&query)
        .unwrap();
    let mut ret = stmt
        .query(NO_PARAMS)
        .unwrap();
    match ret
        .next()
        .unwrap() {
        Some(row) => match row.get::<_, u32>(0) {
            Ok(max_post_id) => max_post_id + 1,
            _ => 1 // It can happen when its type is Null.
        }
        None => 1,
    }
}

/// [Opcode]
/// 0: TODO
/// 1: Erase the list
/// 2: TODO
/// 3: TODO
fn make_packet_for_write_article(err_kind: Option<ErrorKind>, opcode: u8) -> Vec<u8> {
    let err_code = match err_kind {
        Some(v) => v as u32,
        None => 0
    };
    PacketWriter::new(PACKET_WRITE_ARTICLE + 1)
        .u32(err_code) // Error code can be sent.
        .u8(opcode)                      // TODO: opcode
                                    // 1: erase the list
        .as_vec()
}

fn write_article(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let write_mode = pr.u8();   // write mode
                                    // 0: new post
                                    // 1: modify
                                    // 2: delete
                                    // 3: reply
    pr.seek(3);
    let post_id = pr.u32(); // 0 when it is a new post.
    let title = pr.string(49);
    let text = pr.string(2051);

    let db = &server.db;

    match write_mode {
        0 => { // new post
            if post_id != 0 {
                // TODO: Implement
                let pkt = make_packet_for_write_article(Some(ErrorKind::BoardNotReady), 0);
                entity::session::send_by_eid(&server.world, entity_id, pkt);
                return Ok(());
            }

            let id = server
                .world
                .get(entity_id)?
                .get::<AfterLoginInfo>()?
                .user_schema
                .id
                .borrow() as &String;

            let post_id = assign_new_post_id(db);

            let post_entity = PostSchema {
                post_id: post_id,
                parent_post_id: if_else(post_id == 0, None, Some(post_id)),
                writer_id: (*id).clone(),
                title: title,
                text: text,
                datetime: DateTimeSchema::now(),
                is_deleted: false,
                is_notice: false,
                views: 0,
            };

            db
                .table(POST_TBL)?
                .insert(post_entity, &db)?;

        },
        1 => { // modify
            // TODO: Check if the post exists.
            // Check its writer.
            // TODO

        },
        2 => { // delete
            // TODO: Check if the post exists.
            /*
            db
                .table(POST_TBL)?
                .iter()
                .filter(field("post_id").eq(post_id))
                .set("is_deleted", true, db)?;
            */
        },
        3 => { // reply
            // TODO: Check if the post exists.
            

        },
        _ => {
            bail!("Invalid opcode for write_article")
        }
    }

    let pkt_remove_list = make_packet_for_write_article(None, 1);
    let pkt_board_info = get_board_info_pkt(server, 0);
    entity::session::send_by_eid(&server.world, entity_id, pkt_remove_list);
    entity::session::send_by_eid(&server.world, entity_id, pkt_board_info);
    Ok(())
}

fn update_settings(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let key_binding = pr.u8();
    let macros = (0..8)
        .map(|_| pr.string(70))
        .collect();
    pr.seek(3);
    let bgm = pr.u32();
    let sound = pr.u32();

    let settings = SettingSchema {
        bgm_volume: (bgm & 0xff) as u8,
        bgm_mute: if_else((bgm >> 8) & (1 << 0) != 0, false, true),
        bgm_echo: if_else((bgm >> 8) & (1 << 1) != 0, true, false),
        sound_volume: (sound & 0xff) as u8,
        sound_mute: if_else((sound >> 8) & (1 << 0) != 0, false, true),
        key_binding: key_binding,
        macros: macros
    };

    server
        .world
        .get_mut(entity_id)?
        .get_mut::<AfterLoginInfo>()?
        .user_schema
        .setting = settings.clone();

    let id = &server
        .world
        .get(entity_id)?
        .get::<AfterLoginInfo>()?
        .user_schema
        .id;

    {
        let db = &server.db;
        let tbl = db.table(USER_TBL)?;
    
        tbl
            .iter()
            .filter(field("id").eq(id))
            .patch(json!({
                "setting": settings
            }), db)?;
    }

    Ok(())
}

/// Send the client a list of rooms.
fn enter_lobby(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    
    let is_waiting_only = if_else(pr.u8() == 1, true, false);
    let mode = pr.u8(); // 0: From the beginning, 1: >= (forward), 2: <= (backward)
    let base_room_id = pr.u8();
    
    {
        let entity = world.get_mut(entity_id).unwrap();
        // Move into the lobby.
        entity.push(ClientState::OnLobby);
    }

    let rooms = entity::room::get_room_list(world, is_waiting_only, mode, base_room_id);
    
    {
        let entity = world.get(entity_id).unwrap();
        let pkt = packet::room_list(world, &rooms);
        entity::session::send(&entity, pkt);
    }

    Ok(())
}

fn make_packet_for_change_nickname(err_kind: Option<ErrorKind>) -> Vec<u8> {
    let err_code = match err_kind {
        Some(err_kind) => err_kind as u32,
        None => 0
    };
    PacketWriter::new(PACKET_CHANGE_NICKNAME + 1)
        .u32(err_code)
        .as_vec()
}

fn change_nickname(server: &mut Server, eid: &EntityId, mut pr: PacketReader) -> Result<()> {
    let world = &mut server.world;
    let db = &server.db;

    let name = pr.string(12+1);
    let prev_name = world
        .get(eid)?
        .get::<AfterLoginInfo>()?
        .user_schema
        .name
        .clone();

    if name == prev_name {
        let pkt = make_packet_for_change_nickname(Some(ErrorKind::SameNickname));
        entity::session::send_by_eid(world, eid, pkt);
        return Ok(());
    }

    // TODO: Check validity of the name.
    match db
        .as_ref()
        .query_row(&format!("SELECT COUNT(*) from {USER_TBL} WHERE name = ?"), params![&name], |row| row.get(0)) {
        Ok(0) | Err(_) => {},
        _ => {
            let pkt = make_packet_for_change_nickname(Some(ErrorKind::DupNickname));
            entity::session::send_by_eid(world, eid, pkt);
            return Ok(());
        }
    }

    world
        .get_mut(eid)?
        .get_mut::<AfterLoginInfo>()?
        .user_schema
        .name = name.clone();

    db
        .table(USER_TBL)?
        .iter()
        .filter(field("name").eq(prev_name)) // It is okay since uniqueness of names are ensured.
        .set("name", &name, db)?;
    
    let pkt = make_packet_for_change_nickname(None);
    entity::session::send_by_eid(world, eid, pkt);
    
    Ok(())
}


pub fn handle(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    match pr.opcode() {
        PACKET_REQUEST_NOTICE => {
            send_notice(server, entity_id, pr)
        },
        PACKET_REQUEST_TICKER => {
            send_ticker(server, entity_id, pr)
        },
        PACKET_ENTER_BOARD => {
            enter_board(server, entity_id, pr)
        },
        PACKET_READ_ARTICLE => {
            read_article(server, entity_id, pr)
        },
        PACKET_WRITE_ARTICLE => {
            write_article(server, entity_id, pr)
        },
        PACKET_UPDATE_SETTINGS => {
            update_settings(server, entity_id, pr)
        },
        PACKET_ENTER_LOBBY => {
            enter_lobby(server, entity_id, pr)
        },
        PACKET_CHANGE_NICKNAME => {
            change_nickname(server, entity_id, pr)
        },
        opcode => {
            println!("Unknown packet, AfterLogin: {opcode}");
            Ok(())
        }
    }
}