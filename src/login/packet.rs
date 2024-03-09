
use crate::login::*;

use crate::packet::incoming::InPacket;

use self::event::LoginEvent;

#[derive(Event)]
pub struct PacketEvent {
    #[event(target)]
    pub entity: EntityId,
    pub pkt: InPacket,
}

pub fn handle_packet_event(
    receiver: Receiver<PacketEvent>,
    mut login_event_sender: Sender<LoginEvent>,
) {
    match &receiver.event.pkt {
        InPacket::LoginRequest { id, pw } => {
            login_event_sender.send(LoginEvent {
                entity: receiver.event.entity,
                id: id.into(),
                pw: pw.into(),
            });
        },
        _ => {
            // ?
        }
    }
}
