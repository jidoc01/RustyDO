use std::io::ErrorKind;

use crate::{packet::{incoming::InPacket, outgoing::OutPacketBuildable, packet_receiver::PacketReceiver}, status::*, world::TaskExecutable};

const STATUS_SERVER_PORT: u16 = 9874;
const MAX_RECV_PER_TICK: usize = 2;

#[derive(Event)]
pub struct StatusServerTickEvent;

#[derive(Event)]
pub struct PacketReceivedEvent {
    pub addr: SocketAddr,
    pub pkt: InPacket,
}

#[derive(Event)]
pub struct PacketSentEvent {
    pub addr: SocketAddr,
    pub pkt: Box<dyn OutPacketBuildable>,
}

pub fn handle_server_tick_event(
    _: Receiver<StatusServerTickEvent>,
    Single(server_socket): Single<&mut ServerSocket>,
    mut sender: Sender<PacketReceivedEvent>,
) {
    let mut buf = [0u8; 1024];
    let mut packet_receiver = PacketReceiver::default();
    for _ in 0..MAX_RECV_PER_TICK {
        match server_socket.socket.try_recv_from(&mut buf) {
            Ok((n, addr)) => {
                println!("Received {} bytes from {}", n, addr);
                let buf = &buf[0..n];
                packet_receiver.clear();
                packet_receiver.push(buf);
                let Ok(Some(body)) = packet_receiver.try_fetch_body() else {
                    // TODO: block the IP
                    continue;
                };
                let pkt = InPacket::parse(body);
                sender.send(PacketReceivedEvent {
                    addr,
                    pkt,
                });
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                return;
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

/// TODO: detect DoS & block the IP
pub fn handle_packet_received_event(
    receiver: Receiver<PacketReceivedEvent>,
    mut sender: Sender<PacketSentEvent>,
) {
    let mut buf = [0u8; 1024];
    let mut packet_receiver = PacketReceiver::default();
    match receiver.event.pkt {
        InPacket::RequestServerStatus => {
            println!("Received a request from {}", receiver.event.addr);
            /*
            let response_pkt = ();
            sender.send(PacketSentEvent {
                addr: receiver.event.addr,
                pkt: response_pkt
            });
            */
        },
        _ => {
            // ?
        }
    }
}

pub fn handle_packet_sent_event(
    mut receiver: Receiver<PacketSentEvent>,
    Single(mut server_socket): Single<&mut ServerSocket>,
) {
    /*
    let mut writer = Writer::from_vec_mut(&mut v);
    if pkt.try_build(&mut writer).is_err() {
        // ?
        return;
    }
    let (receive_task) = receiver.query;
    let a = receive_task.0;
    */
}

#[derive(Component)]
pub struct ServerSocket {
    socket: tokio::net::UdpSocket
}

impl ServerSocket {
    pub fn new() -> Self {
        let addr = format!("0.0.0.0:{}", STATUS_SERVER_PORT);
        let socket = block_on(tokio::net::UdpSocket::bind(addr)).unwrap();
        info!("Status server listening on port {}", STATUS_SERVER_PORT);
        Self {
            socket
        }
    }
}

#[derive(Default)]
pub struct StatusServerTicker;

impl TaskExecutable for StatusServerTicker {
    const DURATION: Duration = fps_to_duration(2);

    fn init(&mut self, world: &mut World) {
        let e = world.spawn();
        world.insert(e, ServerSocket::new());
    }

    fn execute(
        &mut self,
        world: &mut World,
    ) {
        world.send(StatusServerTickEvent);
    }
}