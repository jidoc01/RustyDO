mod net;

use crate::world::WorldHelper;
pub use crate::prelude::*;

use self::net::{handle_packet_received_event, handle_packet_sent_event, handle_server_tick_event, PacketReceivedEvent, PacketSendEvent, ServerSocket, StatusServerTickEvent, StatusServerTicker};

pub fn init(world_helper: &mut WorldHelper) {
    world_helper
        .add_event::<PacketReceivedEvent>()
        .add_event::<PacketSendEvent>()
        .add_event::<StatusServerTickEvent>();
    world_helper
        .add_task(StatusServerTicker::default());
    world_helper
        .add_system(handle_server_tick_event)
        .add_system(handle_packet_received_event)
        .add_system(handle_packet_sent_event);
}
