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

use super::*;

/// We register server-wide timers here.
/// Currently we use ping-pong checker only.
pub fn register_timers(world: &mut World) {
    add_ping_pong_checker(world);
}

fn add_ping_pong_checker(world: &mut World) {
    let duration = Duration::from_secs(60 * 10); // 10 mins.
    register_timer!(world, duration, move |server: &mut Server, _timer_eid: &EntityId| -> bool {
        for entity in server.world.select_mut(|entity| *entity.get::<EntityKind>().unwrap() == EntityKind::Client) {
            if let Ok(pong_info) = entity.get::<PongInfo>() {
                if let PongInfo(false) = pong_info {
                    // Send a packet to let the client know his leaving.
                    let pkt = handler::packet::error(ErrorKind::NoResponse);
                    entity::session::send(entity, pkt);

                    // Make the connection shut down.
                    let _ = entity
                        .get::<MsgToConnSender>()
                        .unwrap()
                        .send(MsgToConn::Shutdown);
                    continue;
                }
            }
            entity.push(PongInfo(false)); // Reset the state.
        }
        return true;
    });
}