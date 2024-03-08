use evenio::rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator};

use crate::*;

use self::{packet::incoming::InPacket, world::TaskExecutable};

use super::{component::*, packet::PacketEvent, ClientJob};

/// The maximum number of client jobs to be processed per tick.
/// To assure consistency, we limit it to 1 so that we can expect that
/// a client handles only one job per tick.
///
/// TODO: we could scale this number up for cases where we need to handle a large
/// number of packets in a short time (e.g. in-game movement, etc).
const MAX_CLIENT_JOB_PER_TICK: usize = 4;

/// This only generates the `ClientJobTick` event.
pub struct ClientJobTickerTask;

#[derive(Event)]
pub struct ClientJobTick;

impl TaskExecutable for ClientJobTickerTask {
    const DURATION: Duration = fps_to_duration(30);
    fn init(&mut self, _: &mut World) {}
    fn execute(&mut self, world: &mut World) {
        world.send(ClientJobTick);
    }
}

/// Handle messages from ClientSessions.
/// We ignore the clients who are already being disconnected.
pub fn handle_client_job(
    _: Receiver<ClientJobTick>,
    mut fetcher: Fetcher<(EntityId, &mut ClientJobReceiver, Option<&ClientDisconnecting>)>,
    mut sender: Sender<(PacketEvent, Insert<ClientDisconnecting>)>,
) {
    fetcher
        .iter_mut()
        .for_each(|(e, rx, disconnecting)| {
            if disconnecting.is_some() { // ignore disconnecting clients
                return;
            }
            let mut count = 0;
            while let Ok(msg) = rx.0.try_recv() { // TODO: use a global queue.
                match msg {
                    ClientJob::OnReceive(pkt) => sender.send(PacketEvent { entity: e, pkt }),
                    ClientJob::OnDisconnected => sender.insert(e, ClientDisconnecting)
                }
                count += 1;
                if count >= MAX_CLIENT_JOB_PER_TICK {
                    break;
                }
            }
        });
}
