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