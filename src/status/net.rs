use std::io::ErrorKind;

use evenio::handler::Local;

use crate::{encrypt::{encrypt_body, encrypt_header}, packet::{build_packet, incoming::InPacket, outgoing::{OutPacketBuildable, ServerStatusResponse}, packet_receiver::PacketReceiver}, status::*, world::TaskExecutable};

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
pub struct PacketSendEvent {
    pub addr: SocketAddr,
    pub pkt: Box<dyn OutPacketBuildable>,
}

pub fn handle_server_tick_event(
    _: Receiver<StatusServerTickEvent>,
    Single(server_socket): Single<&mut ServerSocket>,
    mut sender: Sender<PacketReceivedEvent>,
) {
    let mut packet_receiver = PacketReceiver::default();
    for _ in 0..MAX_RECV_PER_TICK {
        match server_socket.socket.try_recv_from(&mut server_socket.receive_buffer) {
            Ok((_, addr)) if server_socket.is_blocked(&addr) => {
                debug!("Ignore a packet from a blocked IP: {}", addr);
            },
            Ok((n, addr)) => {
                info!("Received {} bytes from {}", n, addr);
                let buf = &server_socket.receive_buffer[0..n];
                packet_receiver.clear();
                packet_receiver.push(buf);
                // TODO: no need to allocate a new buffer for the body.
                // Instead we can slice the packet buffer w/ proper lifetime.
                let Ok(Some(body)) = packet_receiver.try_fetch_body() else {
                    // TODO: block the IP
                    server_socket.block_ip(addr);
                    continue;
                };
                debug!("body: {:?}", body);
                let pkt = InPacket::parse(body);
                sender.send(PacketReceivedEvent {
                    addr,
                    pkt,
                });
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => { // nothing to read
                break;
            },
            Err(e) => {
                warn!("{}", e);
            },
            _ => {}
        }
    }
}

/// TODO: detect DoS & block the IP
pub fn handle_packet_received_event(
    receiver: Receiver<PacketReceivedEvent>,
    mut sender: Sender<PacketSendEvent>,
) {
    let pkt = &receiver.event.pkt;
    let addr = &receiver.event.addr;
    debug!("Received a packet {:?} from {}", pkt, addr);
    match pkt {
        InPacket::ServerStatusRequest { code } => {
            sender.send(PacketSendEvent {
                addr: addr.clone(),
                pkt: Box::new(ServerStatusResponse { code: *code }),
            });
        },
        _ => {
            debug!("Received an unhandled packet {:?} from {}", pkt, receiver.event.addr);
        }
    }
}

pub fn handle_packet_sent_event(
    receiver: Receiver<PacketSendEvent>,
    Single(server_socket): Single<&ServerSocket>,
) {
    match server_socket.send(receiver.event.addr, &receiver.event.pkt) {
        Ok(_) => {},
        Err(e) => {
            warn!("Sent a packet to {} failed: {}", receiver.event.addr, e);
        },
    }
}

#[derive(Component)]
pub struct ServerSocket {
    socket: tokio::net::UdpSocket,
    receive_buffer: [u8; 1024],
    blocked_addrs: HashSet<SocketAddr>, // TODO: manage a ban list in the db
}

impl ServerSocket {
    pub fn new() -> Self {
        let addr = format!("0.0.0.0:{}", STATUS_SERVER_PORT);
        let socket = block_on(tokio::net::UdpSocket::bind(addr)).unwrap();
        info!("Status server listening on port {}", STATUS_SERVER_PORT);
        Self {
            socket,
            receive_buffer: [0u8; 1024],
            blocked_addrs: HashSet::new(),
        }
    }

    pub fn is_blocked(&self, addr: &SocketAddr) -> bool {
        self.blocked_addrs.contains(addr)
    }

    pub fn block_ip(&mut self, addr: SocketAddr) {
        self.blocked_addrs.insert(addr);
    }

    pub fn send(&self, addr: SocketAddr, pkt: &Box<dyn OutPacketBuildable>) -> anyhow::Result<()> {
        let chunk = build_packet(pkt.as_ref())?;
        self.socket.try_send_to(&chunk, addr)?;
        Ok(())
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