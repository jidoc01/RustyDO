// Copyright 2022 JungHyun Kim
// This file is part of Foobar.
// RustyDO is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// RustyDO is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with RustyDO. If not, see <https://www.gnu.org/licenses/>.

use crate::prelude::*;
use super::*;

const PACKET_LOGIN: u8              = 3;
const PACKET_REENTER_SERVER: u8     = 20;

fn try_get_user_schema(db: &Connection, id: &String, pw: &String) -> (Option<UserSchema>, Option<ErrorKind>) {
    let hashed_pw = hex::encode(hash::from_str(pw));
    let tbl = db.table(USER_TBL).unwrap();
    let found_user_schemas: Vec<UserSchema> = tbl
        .iter()
        .filter(field("id").eq(id))
        .data(&db)
        .unwrap();
    match found_user_schemas.first() {
        Some(user_schema) => {
            // Check if its pw is correct.
            let actual_pw = &user_schema.pw;
            if *actual_pw != hashed_pw { // Invalid password.
                (None, Some (ErrorKind::InvalidAccInfo))
            }
            // Check if it is banned.
            else if user_schema.is_banned {
                (None, Some (ErrorKind::Banned))
            } else {
                (Some((*user_schema).clone()), None)
            }
        },
        _ => (None, Some(ErrorKind::InvalidId)) // No id.
    }
}

/// Try to register a new client uid.
/// It costs O(NlogN).
fn try_register_client_uid(world: &World) -> Option<ClientId> {
    let id_set = {
        let mut id_set = HashSet::new();
        world
            .values()
            .iter()
            .for_each(|&entity| {
                match entity.get::<AfterLoginInfo>() {
                    Ok(info) => { id_set.insert(info.client_uid); },
                    _ => {}
                }
            });
        id_set
    };

    for candidate in MIN_CLIENT_UID..=MAX_CLIENT_UID {
        if !id_set.contains(&candidate) {
            return Some(candidate);
        }
    }
    return None;
}

fn make_login_packet(info: &AfterLoginInfo) -> Vec<u8> {
    let schema = &info.user_schema;
    let setting = &info.user_schema.setting;
    let bgm_setting: u32 =
          if_else(!setting.bgm_mute, 1u32 << 8, 0u32)
        | if_else(setting.bgm_echo, 1u32 << 9, 0u32)
        | setting.bgm_volume as u32;
    let sound_setting: u32 = 
          if_else(!setting.sound_mute, 1u32 << 8, 0)
        | setting.sound_volume as u32;
    let allow_nickname_mod = true;

    let mut pw = PacketWriter::new(PACKET_LOGIN + 1);
    pw
        .u16(info.client_uid)         // Unique number among clients.
        .u8(0)                       // Unknown: (1 << 1) or (1 << 3) or both or none.
        .u8(if_else(schema.is_muted, 1, 0))    // Chat forbidden.
        .pad_to(16)
        .u8(0)
        .u8(schema.level)              // Level.
        .u32(0)
        .u32(0)
        .u8(0)
        .u8(0)
        .string(&schema.name, 16)          // User name.
        .u8(setting.key_binding);	// 0: None, 1~4: Type A~D.
    setting.macros.iter().for_each(|msg| {
        pw
            .string(&msg, 55)
            .pad(15);
    });
    pw
        .pad_to(608)
        .u32(bgm_setting) // bgm setting
        .u32(sound_setting) // sound setting
        .u32(schema.money);
    schema.exps.iter().for_each(|exp| {
        pw.u32(*exp);
    });
    schema.items.iter().for_each(|item_no| {
        pw.u8(*item_no);
    });
    /* TODO: Messanger infos */
    let friend_count = 10;
    pw
        .u8(friend_count)
        .pad_to(660);
    for i in 0..friend_count {
        pw
            .u8(0)
            .u8(1)
            .u32(0)
            .u32(0)
            .u8(0)
            .u8(0)
            .string(&format!("User {i}"), 16);
    }
    pw.pad_to(1220);
    for i in 0..20 { // Unknown
        pw.string(&format!("Test {i}"), 13);
    }
    pw
        .u32(if_else(allow_nickname_mod, 1, 0))
        .u32(0)
        .u8(25)		// 25d.
        .u8(7)		// 7m.
        .u16(2004)	// 2004y.
        .as_vec()
}

fn login(server: &mut Server, entity_id: &EntityId, mut pr: PacketReader) -> Result<()> {
    let id = pr.string(13);
    let password = pr.string(13);

    log!("[{}] Login request received.", entity_id);
    
    let use_auto_account = server
        .config
        .server
        .use_auto_account;
    // Check if the account information is valid. 
    let user_schema = match try_get_user_schema(&server.db, &id, &password) {
        (Some(v), None) => v,
        (None, Some(ErrorKind::InvalidId)) if use_auto_account => {
            // Automatically generate an account.
            let name = id.clone();
            match server.add_new_account(id.clone(), name, password.clone()).as_str() {
                "id_dup" => todo!(),
                "name_dup" => {
                    let pkt = packet::error(ErrorKind::DupNickname);
                    let entity = server.world.get(entity_id).unwrap();
                    entity::session::send(entity, pkt);
                    return Ok(());
                },
                _ => {
                    // Re-try.
                    match try_get_user_schema(&server.db, &id, &password) {
                        (Some(v), None) => v,
                        _ => todo!() // Impossible.
                    }
                }
            }
        },
        (None, Some(err_kind)) => {
            let pkt = packet::error(err_kind);
            let entity = server.world.get(entity_id).unwrap();
            entity::session::send(entity, pkt);
            return Ok(());
        },
        _ => todo!()
    };

    // Check if it's online.
    let is_offline = server
        .world
        .select(|entity| {
            if let Ok(info) = entity.get::<AfterLoginInfo>() {
                info.user_schema.id == id
            } else {
                false
            }
        })
        .is_empty();

    if !is_offline {
        let pkt = packet::error(packet::ErrorKind::Online);
        let entity = server.world.get(entity_id).unwrap();
        entity::session::send(entity, pkt);
        return Ok(());
    }
    
    // Try to get an unique id which will be used on the client-side.
    // Note that it's for distinguishing other clients from a client on
    // the client-side.
    let client_id = try_register_client_uid(&server.world);
    if client_id.is_none() { // Full of clients.
        let pkt = packet::error(packet::ErrorKind::FullOfClients);
        let entity = server.world.get(entity_id).unwrap();
        entity::session::send(entity, pkt);
        return Ok(());
    }
    let client_id = client_id.unwrap();

    // Add components.
    let info = component::AfterLoginInfo::new(
        client_id, 
        user_schema
    );
    let entity = server.world.get_mut(entity_id).unwrap();
    entity.push(ClientState::AfterLogin);
    entity.push(info.clone());

    // Send a packet including various information of the user.
    let pkt = make_login_packet(&info);
    entity::session::send(&entity, pkt);

    log!("[{}] Login accepted.", entity_id);

    Ok(())
}

fn reenter_server(server: &mut Server, entity_id: &EntityId, _pr: PacketReader) -> Result<()> {
    let entity =
        server
            .world
            .get_mut(entity_id)?;
    // Check if it is authenticated.
    if entity.get::<AfterLoginInfo>().is_err() {
        bail!("Invalid access: it tried to re-enter the server, but was not authenticated");
    }
    entity.push(ClientState::AfterLogin);

    Ok(())
}

pub fn handle(server: &mut Server, entity_id: &EntityId, pr: PacketReader) -> Result<()> {
    match pr.opcode() {
        PACKET_LOGIN => { // Login request.
            login(server, entity_id, pr)
        },
        PACKET_REENTER_SERVER => {
            reenter_server(server, entity_id, pr)
        },
        opcode => {
            println!("Unknown packet, BeforeLogin: {opcode}");
            Ok(())
        }
    }
}