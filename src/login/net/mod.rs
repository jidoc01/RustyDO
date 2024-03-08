mod client_session;
mod client_job;

use std::time::Duration;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, runtime::Handle, select, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::block_in_place};
use self::{client_job::{handle_client_job, ClientJobTickerTask}, client_session::run_client_session_async};

use crate::{login::*, world::TaskExecutable};

const SERVER_PORT: u16 = 9874;
const MAX_ACCEPT_PER_TICK: usize = 2;
const FPS: u32 = 10; // 10 acceptances per second

struct AcceptInfo {
    stream: TcpStream,
    addr: std::net::SocketAddr,
}

pub fn init(world_helper: &mut WorldHelper) {
    let rx = block_on(listen_and_get_rx());
    world_helper
        .add_task(AcceptHandlerTask::new(rx))
        .add_task(ClientJobTickerTask);
    world_helper
        .add_system(handle_client_job);
}

async fn listen_and_get_rx() -> UnboundedReceiver<AcceptInfo> {
    let (tx, rx) = unbounded_channel();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", SERVER_PORT)).await.unwrap();
    info!("Login server listening on port {}", SERVER_PORT);
    tokio::spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            tx.send(AcceptInfo { stream, addr }).unwrap();
        }
    });
    return rx;
}

struct AcceptHandlerTask {
    rx: UnboundedReceiver<AcceptInfo>,
}

impl AcceptHandlerTask {
    fn new(rx: UnboundedReceiver<AcceptInfo>) -> Self {
        Self { rx }
    }
}

impl TaskExecutable for AcceptHandlerTask {
    const DURATION: Duration = fps_to_duration(1);
    fn init(&mut self, _: &mut World) {}
    fn execute(&mut self, world: &mut World) {
        let mut count = 0;
        while let Ok(AcceptInfo { stream, addr }) = self.rx.try_recv() {
            count += 1;
            if count > MAX_ACCEPT_PER_TICK {
                break;
            }

            let entity = world.spawn();
            let (client_tx, client_rx) = unbounded_channel();
            let (client_session_tx, client_session_rx) = unbounded_channel();
            tokio::spawn(async move {
                run_client_session_async(stream, client_tx, client_session_rx).await;
            });

            let client = world.spawn();
            world.insert(entity, ClientAddr(addr));
            world.insert(entity, ClientSessionJobSender(client_session_tx));
            world.insert(entity, ClientJobReceiver(client_rx));
        }
    }
}